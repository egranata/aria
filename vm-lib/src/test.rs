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
fn test_local_define_type_mismatch() {
    let input = r##"
func main() {
    val x: Int = "a";
    assert(x);
}
"##;

    assert!(exec_code(input).is_err_and(|err| err.reason == VmErrorReason::TypecheckFailed));
}

#[test]
fn test_local_write_type_mismatch() {
    let input = r##"
func main() {
    val x: Int = 1;
    x = false;
    assert(x);
}
"##;

    assert!(exec_code(input).is_err_and(|err| err.reason == VmErrorReason::TypecheckFailed));
}

#[test]
fn test_func_argument_type_mismatch() {
    let input = r##"
func add(x: Int, y: Int) {
    return x + y;
}

func main() {
    val x = add(3,"hello");
    assert(false);
}
"##;

    assert!(exec_code(input).is_err_and(|err| err.reason == VmErrorReason::TypecheckFailed));
}

#[test]
fn test_method_argument_type_mismatch() {
    let input = r##"
struct Adder {
    type func new(x) {
        return alloc(Adder){.x = x,};
    }

    instance func add(x: Int) {
        this.x + x;
    }
}

func main() {
    val a = Adder.new(4);
    val n = a.add(false);
    assert(false);
}
"##;

    assert!(exec_code(input).is_err_and(|err| err.reason == VmErrorReason::TypecheckFailed));
}

#[test]
fn test_func_union_type_mismatch() {
    let input = r##"
func id(x: Int|String) {
    x;
} func main() {
    val x = id([]);
    assert(false);
}
"##;

    assert!(exec_code(input).is_err_and(|err| err.reason == VmErrorReason::TypecheckFailed));
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

    assert!(exec_code(input).is_err_and(|err| err.reason == VmErrorReason::TypecheckFailed));
}

#[test]
fn test_err_in_op_is_caught() {
    let input = r##"
struct Foo {
    func op_add(x) {
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
    func op_radd(x) {
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
                tracing: false,
                dump_stack: false,
                vm_args: vec!["foo".to_owned(), "bar".to_owned()],
            }
        )
        .is_ok()
    );
}
