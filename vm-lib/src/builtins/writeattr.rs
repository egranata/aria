// SPDX-License-Identifier: Apache-2.0
use crate::{
    builtins::VmBuiltins, error::vm_error::VmErrorReason, frame::Frame, ok_or_err,
    runtime_value::function::BuiltinFunctionImpl, vm::RunloopExit,
};

#[derive(Default)]
struct WriteAttr {}
impl BuiltinFunctionImpl for WriteAttr {
    fn eval(
        &self,
        frame: &mut Frame,
        vm: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let the_object = frame.stack.pop();
        let the_string = VmBuiltins::extract_arg(frame, |x| x.as_string().cloned())?;
        let the_value = frame.stack.pop();
        let result = the_object.write_attribute(&the_string.raw_value(), the_value);
        match result {
            Ok(_) => {
                frame.stack.push(ok_or_err!(
                    vm.builtins.create_unit_object(),
                    VmErrorReason::UnexpectedVmState.into()
                ));
                Ok(RunloopExit::Ok(()))
            }
            Err(e) => {
                let er = match e {
                    crate::runtime_value::AttributeError::NoSuchAttribute => {
                        VmErrorReason::NoSuchIdentifier(the_string.raw_value())
                    }
                    crate::runtime_value::AttributeError::InvalidFunctionBinding => {
                        VmErrorReason::InvalidBinding
                    }
                    crate::runtime_value::AttributeError::ValueHasNoAttributes => {
                        VmErrorReason::UnexpectedType
                    }
                };
                Err(er.into())
            }
        }
    }

    fn required_argc(&self) -> u8 {
        3_u8
    }

    fn name(&self) -> &str {
        "writeattr"
    }
}

pub(super) fn insert_builtins(builtins: &mut VmBuiltins) {
    builtins.insert_builtin::<WriteAttr>();
}
