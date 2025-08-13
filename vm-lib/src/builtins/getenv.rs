// SPDX-License-Identifier: Apache-2.0
use crate::{
    builtins::VmBuiltins,
    frame::Frame,
    runtime_value::{RuntimeValue, function::BuiltinFunctionImpl},
    vm::RunloopExit,
};

#[derive(Default)]
struct Getenv {}
impl BuiltinFunctionImpl for Getenv {
    fn eval(
        &self,
        frame: &mut Frame,
        vm: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let var_name = VmBuiltins::extract_arg(frame, |x| x.as_string().cloned())?;
        match std::env::var(var_name.raw_value()).map(|s| RuntimeValue::String(s.into())) {
            Ok(s) => match vm.builtins.create_maybe_some(s) {
                Ok(s) => {
                    frame.stack.push(s);
                    Ok(RunloopExit::Ok(()))
                }
                Err(e) => Err(e.into()),
            },
            Err(_) => match vm.builtins.create_maybe_none() {
                Ok(s) => {
                    frame.stack.push(s);
                    Ok(RunloopExit::Ok(()))
                }
                Err(e) => Err(e.into()),
            },
        }
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "getenv"
    }
}

pub(super) fn insert_builtins(builtins: &mut VmBuiltins) {
    builtins.insert_builtin::<Getenv>();
}
