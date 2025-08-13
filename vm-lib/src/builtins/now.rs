// SPDX-License-Identifier: Apache-2.0
use crate::{
    builtins::VmBuiltins,
    frame::Frame,
    runtime_value::{RuntimeValue, function::BuiltinFunctionImpl},
    vm::RunloopExit,
};

#[derive(Default)]
struct Now {}
impl BuiltinFunctionImpl for Now {
    fn eval(
        &self,
        cur_frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("before the epoch")
            .as_millis() as i64;
        cur_frame.stack.push(RuntimeValue::Integer(now.into()));
        Ok(RunloopExit::Ok(()))
    }

    fn required_argc(&self) -> u8 {
        0_u8
    }

    fn name(&self) -> &str {
        "now"
    }
}

pub(super) fn insert_builtins(builtins: &mut VmBuiltins) {
    builtins.insert_builtin::<Now>();
}
