// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use aria_compiler::{compile_from_ast, do_compile::CompilationError, CompilationOptions};
use aria_parser::ast::{
    prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
    source_to_ast, ParserError, SourceBuffer, SourcePointer,
};
use ariadne::{Color, Label, Report, ReportKind, Source};
use clap::Parser;
use haxby_vm::{
    error::{
        exception::VmException,
        vm_error::{VmError, VmErrorReason},
    },
    frame::Frame,
    runtime_module::RuntimeModule,
    vm::{VirtualMachine, VmOptions},
};

#[derive(Default, Debug, Clone)]
pub struct StringCache {
    buffers: HashMap<String, Source>,
}

impl ariadne::Cache<&String> for StringCache {
    type Storage = String;

    fn fetch(&mut self, path: &&String) -> Result<&Source, Box<dyn std::fmt::Debug + '_>> {
        Ok(&self.buffers[*path])
    }
    fn display<'a>(&self, path: &&'a String) -> Option<Box<dyn std::fmt::Display + 'a>> {
        Some(Box::new(*path))
    }
}

fn report_from_msg_and_location(msg: &str, locations: &[&SourcePointer]) {
    let magenta = Color::Magenta;
    let primary_span = &locations[0];
    let mut report = Report::build(
        ReportKind::Error,
        (
            &primary_span.buffer.name,
            primary_span.location.start..primary_span.location.stop,
        ),
    )
    .with_message(msg);
    let mut cache = StringCache::default();
    for loc in locations {
        report = report.with_label(
            Label::new((&loc.buffer.name, loc.location.start..loc.location.stop))
                .with_message("here")
                .with_color(magenta),
        );
        if !cache.buffers.contains_key(&loc.buffer.name) {
            cache.buffers.insert(
                loc.buffer.name.clone(),
                Source::from((*loc.buffer.content).clone()),
            );
        }
    }
    report.finish().eprint(cache).unwrap();
}

fn report_from_vm_error(err: &VmError) {
    let msg = err.reason.to_string();
    let loc = &err.loc;
    if let Some(loc) = loc {
        report_from_msg_and_location(&msg, &[loc]);
    } else {
        eprintln!("vm execution error: {msg}");
    }
}

fn report_from_vm_exception(vm: &mut VirtualMachine, exc: &VmException) {
    let mut cur_frame = Frame::default();
    let msg = exc.value.prettyprint(&mut cur_frame, vm);
    let backtraces: Vec<_> = exc.backtrace.entries_iter().collect();
    report_from_msg_and_location(&msg, backtraces.as_slice());
}

fn report_from_compiler_error(err: &CompilationError) {
    let msg = err.reason.to_string();
    let loc = &err.loc;
    report_from_msg_and_location(&msg, &[loc]);
}

fn report_from_parser_error(err: &ParserError) {
    let msg = &err.msg;
    let loc = &err.loc;
    report_from_msg_and_location(msg, &[loc]);
}

#[derive(Parser, Debug)]
#[command(author, version, about, trailing_var_arg = true)]
struct Args {
    /// The name of the program file to run
    path: String,
    /// Should the VM trace instruction execution
    #[arg(long("trace-exec"))]
    trace_exec: bool,
    /// Should the VM dump the stack at each instruction
    #[arg(long("trace-stack"))]
    trace_stack: bool,
    /// Should the AST be dumped after parsing
    #[arg(long("dump-ast"))]
    dump_ast: bool,
    /// Should the module be dumped after compilation
    #[arg(long("dump-module"))]
    dump_mod: bool,
    /// Turn off compile-time optimizations
    #[arg(long("disable-optimizer"))]
    disable_optimizer: bool,
    #[arg(trailing_var_arg = true)]
    extra_args: Vec<String>,
}

impl From<&Args> for VmOptions {
    fn from(value: &Args) -> Self {
        let mut options = VmOptions::default();

        if value.trace_exec {
            options.tracing = true;
            if value.trace_stack {
                options.dump_stack = true;
            }
        }

        options.vm_args = value.extra_args.clone();

        options
    }
}

impl From<&Args> for CompilationOptions {
    fn from(value: &Args) -> Self {
        CompilationOptions {
            optimize: !value.disable_optimizer,
        }
    }
}

fn file_eval(path: &str, args: &Args) {
    let mut vm = VirtualMachine::with_options(VmOptions::from(args));

    let buffer = SourceBuffer::file(path);
    match buffer {
        Ok(src) => {
            let _ = eval_buffer(false, &None, src, &mut vm, args);
        }
        Err(err) => {
            println!("error reading source file: {err}");
        }
    }
}

// To permit return Err(report_blah(x)) where report_blah(x) -> ()
#[allow(clippy::unit_arg)]
fn eval_buffer(
    is_repl: bool,
    prior_art: &Option<RuntimeModule>,
    sb: SourceBuffer,
    vm: &mut VirtualMachine,
    args: &Args,
) -> Result<RuntimeModule, ()> {
    let ast = match source_to_ast(&sb) {
        Ok(ast) => ast,
        Err(err) => {
            return Err(report_from_parser_error(&err));
        }
    };

    if args.dump_ast {
        let ast_buffer = PrintoutAccumulator::default();
        let output = ast.prettyprint(ast_buffer).value();
        println!("AST dump:\n{output}\n");
    }

    let comp_opts = CompilationOptions::from(args);

    let c_module = match compile_from_ast(&ast, &comp_opts) {
        Ok(module) => module,
        Err(err) => {
            err.iter().for_each(report_from_compiler_error);
            return Err(());
        }
    };

    if args.dump_mod {
        let mod_buffer = PrintoutAccumulator::default();
        let output = c_module.prettyprint(mod_buffer).value();
        println!("Module dump:\n{output}\n");
    }

    let r_module = RuntimeModule::new(c_module);
    let _ = prior_art.as_ref().is_some_and(|pa| {
        r_module.lift_all_symbols_from_other(pa, vm);
        true
    });

    let r_module = match vm.load_into_module("", r_module) {
        Ok(rle) => match rle {
            haxby_vm::vm::RunloopExit::Ok(m) => m.module,
            haxby_vm::vm::RunloopExit::Exception(exc) => {
                return Err(report_from_vm_exception(vm, &exc));
            }
        },
        Err(err) => {
            return Err(report_from_vm_error(&err));
        }
    };

    let exec_result = vm.execute_module(&r_module);

    match exec_result {
        Ok(rle) => match rle {
            haxby_vm::vm::RunloopExit::Ok(_) => Ok(r_module),
            haxby_vm::vm::RunloopExit::Exception(exc) => Err(report_from_vm_exception(vm, &exc)),
        },
        Err(err) => {
            let is_missing_main = match &err.reason {
                VmErrorReason::NoSuchIdentifier(n) => n == "main",
                _ => false,
            };
            if is_repl && is_missing_main {
                Ok(r_module)
            } else {
                Err(report_from_vm_error(&err))
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    file_eval(&args.path, &args);
}
