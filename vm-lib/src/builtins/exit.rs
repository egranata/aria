// SPDX-License-Identifier: Apache-2.0
use crate::{
    builtins::VmBuiltins, frame::Frame, runtime_value::function::BuiltinFunctionImpl,
    vm::RunloopExit,
};

#[derive(Default)]
struct Exit {}
impl BuiltinFunctionImpl for Exit {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let code = VmBuiltins::extract_arg(frame, |x| x.as_integer().cloned())?.raw_value();
        std::process::exit(code as i32);
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity::required(1)
    }

    fn name(&self) -> &str {
        "exit"
    }
}

pub(super) fn insert_builtins(builtins: &mut VmBuiltins) {
    builtins.insert_builtin::<Exit>();
}
