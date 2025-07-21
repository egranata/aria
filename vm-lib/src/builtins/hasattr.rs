// SPDX-License-Identifier: Apache-2.0
use crate::{
    builtins::VmBuiltins,
    frame::Frame,
    runtime_value::{function::BuiltinFunctionImpl, RuntimeValue},
    vm::RunloopExit,
};

#[derive(Default)]
struct HasAttr {}
impl BuiltinFunctionImpl for HasAttr {
    fn eval(
        &self,
        frame: &mut Frame,
        vm: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let the_value = frame.stack.pop();
        let the_string = VmBuiltins::extract_arg(frame, |x| x.as_string().cloned())?;
        let has_attr = the_value
            .read_attribute(&the_string.raw_value(), &vm.builtins)
            .is_ok();
        frame.stack.push(RuntimeValue::Boolean(has_attr.into()));
        Ok(RunloopExit::Ok(()))
    }

    fn arity(&self) -> u8 {
        2_u8
    }

    fn name(&self) -> &str {
        "hasattr"
    }
}

pub(super) fn insert_builtins(builtins: &mut VmBuiltins) {
    builtins.insert_builtin::<HasAttr>();
}
