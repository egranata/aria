// SPDX-License-Identifier: Apache-2.0
use aria_compiler::compile_from_source;
use aria_parser::ast::SourceBuffer;

use crate::{
    HaxbyEvalResult,
    error::vm_error::VmErrorReason,
    haxby_eval,
    vm::{ExecutionResult, VmOptions},
};

fn exec_code(src: &'static str) -> ExecutionResult<HaxbyEvalResult> {
    exec_code_with_vm_options(src, Default::default())
}

fn exec_code_with_vm_options(
    src: &'static str,
    vm_opts: VmOptions,
) -> ExecutionResult<HaxbyEvalResult> {
    let sb = SourceBuffer::stdin(src);
    let module = compile_from_source(&sb, &Default::default()).expect("module did not compile");
    haxby_eval(module, vm_opts)
}

#[test]
fn test_assert_can_pass() {
    let input = r##"
func main() {
    val x = 1;
    assert x == 1;
}
"##;

    assert!(exec_code(input).is_ok());
}

#[test]
fn test_assert_can_fail() {
    let input = r##"
func main() {
    val x = 1;
    assert x == 2;
}
"##;

    assert!(
        exec_code(input)
            .is_err_and(|err| err.reason == VmErrorReason::AssertFailed("x==2".to_owned()))
    );
}

#[test]
fn test_circular_import_detected() {
    let input = r##"
import circular.zero;

func main() {
    assert false;
}
"##;

    assert!(
        exec_code(input).is_err_and(
            |err| err.reason == VmErrorReason::CircularImport("circular.zero".to_owned())
        )
    );
}

#[test]
fn test_module_val_typecheck_fails() {
    let input = r##"
val x: Int = 1;

func main() {
    x = "false";
    assert false;
}
"##;

    let result = exec_code(input);
    match result {
        Ok(result) => match result.exit {
            crate::vm::RunloopExit::Ok(_) => {
                assert!(false, "expected typecheck to fail");
            }
            crate::vm::RunloopExit::Exception(e) => {
                let enum_value = e
                    .value
                    .as_enum_value()
                    .expect("exception should be an enum value");
                assert!(
                    enum_value.get_case_index()
                        == enum_value
                            .get_container_enum()
                            .get_idx_of_case("UnexpectedType")
                            .expect("enum should have UnexpectedType case")
                );
            }
        },
        Err(_) => {
            assert!(false, "expected exception to be thrown");
        }
    }
}

#[test]
fn test_err_in_op_is_caught() {
    let input = r##"
struct Foo {
    operator + (rhs) {
        assert false;
    }
}

func main() {
    return alloc(Foo) + 1;
}
"##;

    assert!(
        exec_code(input)
            .is_err_and(|err| err.reason == VmErrorReason::AssertFailed("false".to_owned()))
    );
}

#[test]
fn test_err_in_rop_is_caught() {
    let input = r##"
struct Foo {
    reverse operator + (lhs) {
        assert false;
    }
}

func main() {
    return 1 + alloc(Foo);
}
"##;

    assert!(
        exec_code(input)
            .is_err_and(|err| err.reason == VmErrorReason::AssertFailed("false".to_owned()))
    );
}

#[test]
fn test_uncaught_exception_bubbles_up() {
    let input = r##"
func main() {
    throw 1;
}
"##;

    match exec_code(input).expect("ok result expected").exit {
        crate::vm::RunloopExit::Ok(_) => {
            assert!(false);
        }
        crate::vm::RunloopExit::Exception(e) => {
            assert_eq!(
                1,
                e.value
                    .as_integer()
                    .expect("integer value thrown")
                    .raw_value()
            )
        }
    }
}

#[test]
fn test_cmdline_arguments() {
    let input = r##"
func main() {
    val args = cmdline_arguments();
    assert args.len() == 2;
    assert args[0] == "foo";
    assert args[1] == "bar";
}
"##;

    assert!(
        exec_code_with_vm_options(
            input,
            VmOptions {
                vm_args: vec!["foo".to_owned(), "bar".to_owned()],
                ..Default::default()
            }
        )
        .is_ok()
    );
}
