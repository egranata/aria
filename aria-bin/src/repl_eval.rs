// SPDX-License-Identifier: Apache-2.0
use aria_compiler::{CompilationOptions, compile_from_ast};
use aria_parser::ast::{
    ExpressionStatement, SourceBuffer, TopLevelEntry,
    prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    source_to_ast,
};
use haxby_vm::{
    runtime_module::RuntimeModule,
    vm::{VirtualMachine, VmOptions},
};
use reedline::{DefaultPrompt, FileBackedHistory, Reedline, Validator};

use crate::{
    Args,
    error_reporting::{
        report_from_compiler_error, report_from_parser_error, report_from_vm_error,
        report_from_vm_exception,
    },
};

fn matching_closer(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => panic!("unexpected call"),
    }
}

struct ReplValidator;
impl Validator for ReplValidator {
    fn validate(&self, line: &str) -> reedline::ValidationResult {
        use reedline::ValidationResult::Complete;
        use reedline::ValidationResult::Incomplete;

        enum InString {
            Yes(char),
            No,
        }

        impl InString {
            fn next(self, c: char) -> InString {
                match self {
                    InString::Yes(cc) => {
                        if cc == c {
                            InString::No
                        } else {
                            self
                        }
                    }
                    InString::No => Self::Yes(c),
                }
            }

            fn as_bool(&self) -> bool {
                match self {
                    InString::Yes(_) => true,
                    InString::No => false,
                }
            }
        }

        let mut in_string = InString::No;
        let mut balance: Vec<char> = Vec::new();

        for c in line.chars() {
            match c {
                '"' | '\'' => in_string = in_string.next(c),
                '(' | '[' | '{' => {
                    if !in_string.as_bool() {
                        balance.push(matching_closer(c));
                    }
                }
                ')' | ']' | '}' => {
                    if !in_string.as_bool()
                        && let Some(last) = balance.last()
                        && *last == c
                    {
                        balance.pop();
                    }
                }
                _ => {}
            }
        }

        if !in_string.as_bool() && balance.is_empty() {
            Complete
        } else {
            Incomplete
        }
    }
}

struct LineEditor {
    line_editor: Reedline,
    prompt: DefaultPrompt,
}

impl LineEditor {
    pub fn new() -> Self {
        let validator = Box::new(ReplValidator);
        let history = Box::new(
            FileBackedHistory::with_file(1024, "history.aria".into())
                .expect("Error configuring history with file"),
        );
        let prompt = DefaultPrompt {
            left_prompt: reedline::DefaultPromptSegment::Empty,
            right_prompt: reedline::DefaultPromptSegment::Empty,
        };
        Self {
            line_editor: Reedline::create()
                .with_validator(validator)
                .with_history(history),
            prompt,
        }
    }
}

impl LineEditor {
    fn read_input(&mut self) -> (String, bool) {
        let sig = self.line_editor.read_line(&self.prompt);
        match sig {
            Ok(reedline::Signal::Success(buffer)) => (buffer, false),
            Ok(reedline::Signal::CtrlC) | Ok(reedline::Signal::CtrlD) | Err(_) => {
                (String::new(), true)
            }
        }
    }
}

#[allow(clippy::unit_arg)]
fn setup_aria_vm(args: &Args) -> Result<(VirtualMachine, RuntimeModule), ()> {
    let mut vm = VirtualMachine::with_options(VmOptions::from(args));
    let repl_module_preamble = "";

    let sb = SourceBuffer::stdin_with_name(repl_module_preamble, "repl");
    let ast = match source_to_ast(&sb) {
        Ok(ast) => ast,
        Err(err) => {
            return Err(report_from_parser_error(&err));
        }
    };

    let comp_opts = CompilationOptions::default();

    let c_module = match compile_from_ast(&ast, &comp_opts) {
        Ok(module) => module,
        Err(err) => {
            err.iter().for_each(report_from_compiler_error);
            return Err(());
        }
    };

    let r_module = RuntimeModule::new(c_module);

    let r_module = match vm.load_into_module("repl", r_module) {
        Ok(rle) => match rle {
            haxby_vm::vm::RunloopExit::Ok(m) => m.module,
            haxby_vm::vm::RunloopExit::Exception(exc) => {
                return Err(report_from_vm_exception(&mut vm, &exc));
            }
        },
        Err(err) => {
            return Err(report_from_vm_error(&err));
        }
    };

    vm.inject_imported_module("repl", r_module.clone());
    Ok((vm, r_module))
}

fn is_call_to_print_or_println(expr: &ExpressionStatement) -> bool {
    if let Some(val) = &expr.val {
        let result = val.is_function_call();
        if result.0 {
            if let Some(name) = result.1 {
                name == "print" || name == "println"
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

fn massage_ast_for_repl(ast: &mut aria_parser::ast::ParsedModule) -> bool {
    if ast.entries.len() != 2 {
        return false;
    }

    let entry_1 = &ast.entries[1];
    if let TopLevelEntry::ExpressionStatement(expr) = entry_1 {
        if is_call_to_print_or_println(expr) {
            false
        } else if let Some(val) = &expr.val {
            let new_node = val.call_function_passing_me("println");
            ast.entries[1] = TopLevelEntry::ExpressionStatement(ExpressionStatement {
                loc: val.loc().clone(),
                val: Some(new_node),
            });
            true
        } else {
            false
        }
    } else {
        false
    }
}

#[allow(clippy::unit_arg)]
fn process_buffer(
    counter: u64,
    buffer: &str,
    vm: &mut VirtualMachine,
    repl_module: &RuntimeModule,
    args: &Args,
) -> Result<RuntimeModule, ()> {
    let module_name = format!("__repl_chunk_{}", counter);
    let module_source_code = format!("import * from repl;\n{}\n", buffer);
    let sb = SourceBuffer::stdin_with_name(&module_source_code, &module_name);

    let mut ast = match source_to_ast(&sb) {
        Ok(ast) => ast,
        Err(err) => {
            return Err(report_from_parser_error(&err));
        }
    };

    let mutated = massage_ast_for_repl(&mut ast);

    if args.dump_ast {
        let ast_buffer = PrintoutAccumulator::default();
        let output = ast.prettyprint(ast_buffer).value();
        println!("AST dump:\n{output}\n");
        if mutated {
            println!("note: AST mutated for REPL purposes");
        }
    }

    let comp_opts = CompilationOptions::default();

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
    if r_module
        .lift_all_symbols_from_other(repl_module, vm)
        .is_err()
    {
        return Err(());
    }
    match vm.load_into_module("repl", r_module) {
        Ok(rle) => match rle {
            haxby_vm::vm::RunloopExit::Ok(m) => Ok(m.module),
            haxby_vm::vm::RunloopExit::Exception(exc) => Err(report_from_vm_exception(vm, &exc)),
        },
        Err(err) => Err(report_from_vm_error(&err)),
    }
}

pub(crate) fn repl_eval(args: &Args) {
    let (mut vm, repl_module) = setup_aria_vm(args).unwrap();

    let mut ed = LineEditor::new();
    let mut loop_idx: u64 = 0;

    loop {
        let (input, eof) = ed.read_input();
        if eof {
            break;
        }
        if input.trim() == "" {
            continue;
        }
        let new_module = process_buffer(loop_idx, &input, &mut vm, &repl_module, args);
        loop_idx += 1;
        if let Ok(new_module) = new_module {
            let _ = repl_module.lift_all_symbols_from_other(&new_module, &vm);
        }
    }
}
