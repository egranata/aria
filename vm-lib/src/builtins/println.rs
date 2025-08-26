// SPDX-License-Identifier: Apache-2.0
use crate::{
    builtins::VmBuiltins, error::vm_error::VmErrorReason, frame::Frame, ok_or_err,
    runtime_value::function::BuiltinFunctionImpl, vm::RunloopExit,
};

#[derive(Default)]
struct Println {}
impl BuiltinFunctionImpl for Println {
    fn eval(
        &self,
        cur_frame: &mut Frame,
        vm: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        if let Some(the_value) = cur_frame.stack.try_pop() {
            let fmt = the_value.prettyprint(cur_frame, vm);
            assert!(vm.console().borrow_mut().println(&fmt).is_ok());
        } else {
            assert!(vm.console().borrow_mut().println("").is_ok());
        }

        cur_frame.stack.push(ok_or_err!(
            vm.builtins.create_unit_object(),
            VmErrorReason::UnexpectedVmState.into()
        ));
        Ok(RunloopExit::Ok(()))
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity {
            required: 0,
            optional: 1,
        }
    }

    fn name(&self) -> &str {
        "println"
    }
}

pub(super) fn insert_builtins(builtins: &mut VmBuiltins) {
    builtins.insert_builtin::<Println>();
}
