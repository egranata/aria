// SPDX-License-Identifier: Apache-2.0
use crate::{
    builtins::VmBuiltins,
    frame::Frame,
    runtime_value::{RuntimeValue, function::BuiltinFunctionImpl, kind::RuntimeValueType},
    vm::RunloopExit,
};

#[derive(Default)]
struct Typeof {}
impl BuiltinFunctionImpl for Typeof {
    fn eval(
        &self,
        frame: &mut Frame,
        vm: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let the_value = frame.stack.pop();
        let the_type = RuntimeValueType::get_type(&the_value, &vm.builtins);
        frame.stack.push(RuntimeValue::Type(the_type));
        Ok(RunloopExit::Ok(()))
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "typeof"
    }
}

pub(super) fn insert_builtins(builtins: &mut VmBuiltins) {
    builtins.insert_builtin::<Typeof>();
}
