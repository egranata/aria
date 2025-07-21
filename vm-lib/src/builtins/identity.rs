// SPDX-License-Identifier: Apache-2.0
use crate::{
    builtins::VmBuiltins,
    frame::Frame,
    runtime_value::{function::BuiltinFunctionImpl, RuntimeValue},
    vm::RunloopExit,
};

#[derive(Default)]
struct Identity {}
impl BuiltinFunctionImpl for Identity {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let the_value = frame.stack.pop();
        let obj = RuntimeValue::Integer((the_value.identity() as i64).into());
        frame.stack.push(obj);
        Ok(RunloopExit::Ok(()))
    }

    fn arity(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "identity"
    }
}

pub(super) fn insert_builtins(builtins: &mut VmBuiltins) {
    builtins.insert_builtin::<Identity>();
}
