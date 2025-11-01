// SPDX-License-Identifier: Apache-2.0
use std::time::Duration;

use super::VmBuiltins;
use crate::{
    error::vm_error::VmErrorReason, frame::Frame, runtime_value::function::BuiltinFunctionImpl,
    vm::RunloopExit,
};

#[derive(Default)]
struct Sleep {}
impl BuiltinFunctionImpl for Sleep {
    fn eval(
        &self,
        cur_frame: &mut Frame,
        vm: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let duration = VmBuiltins::extract_arg(cur_frame, |x| x.as_integer().cloned())?.raw_value();
        if duration >= 0 {
            std::thread::sleep(Duration::from_millis(duration as u64));
        } else {
            return Err(
                VmErrorReason::OperationFailed("cannot sleep < 0 milliseconds".to_owned()).into(),
            );
        }

        cur_frame.stack.push(vm.builtins.create_unit_object()?);
        Ok(RunloopExit::Ok(()))
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity::required(1)
    }

    fn name(&self) -> &str {
        "sleep_ms"
    }
}

pub(super) fn insert_builtins(builtins: &mut VmBuiltins) {
    builtins.insert_builtin::<Sleep>();
}
