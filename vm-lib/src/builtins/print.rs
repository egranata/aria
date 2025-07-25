// SPDX-License-Identifier: Apache-2.0
use crate::{
    builtins::VmBuiltins, error::vm_error::VmErrorReason, frame::Frame, ok_or_err,
    runtime_value::function::BuiltinFunctionImpl, vm::RunloopExit,
};

#[derive(Default)]
struct Print {}
impl BuiltinFunctionImpl for Print {
    fn eval(
        &self,
        cur_frame: &mut Frame,
        vm: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let the_value = cur_frame.stack.pop();
        print!("{}", the_value.prettyprint(cur_frame, vm));

        cur_frame.stack.push(ok_or_err!(
            vm.builtins.create_unit_object(),
            VmErrorReason::UnexpectedVmState.into()
        ));
        Ok(RunloopExit::Ok(()))
    }

    fn arity(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "print"
    }
}

pub(super) fn insert_builtins(builtins: &mut VmBuiltins) {
    builtins.insert_builtin::<Print>();
}
