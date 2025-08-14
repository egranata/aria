// SPDX-License-Identifier: Apache-2.0

mod error_reporting;
mod file_eval;
mod repl_eval;

use clap::Parser;
use haxby_vm::vm::VmOptions;

#[derive(Parser, Debug)]
#[command(author, version, about, trailing_var_arg = true)]
struct Args {
    /// The name of the program file to run
    path: Option<String>,
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

fn main() {
    let args = Args::parse();
    if let Some(path) = &args.path {
        file_eval::file_eval(path, &args);
    } else {
        repl_eval::repl_eval(&args);
    }
}
