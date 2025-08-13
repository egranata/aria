// SPDX-License-Identifier: Apache-2.0
use super::VmBuiltins;
use crate::{
    error::vm_error::VmErrorReason,
    frame::Frame,
    runtime_value::{RuntimeValue, function::BuiltinFunctionImpl, integer::IntegerValue},
    vm::RunloopExit,
};
use std::process::Command;

fn get_shell_path() -> String {
    // it's not like Aria is tested on Windows, but let's at least pretend to be nice
    if cfg!(target_os = "windows") {
        return std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string());
    }
    std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())
}

#[derive(Default)]
struct System {}
impl BuiltinFunctionImpl for System {
    fn eval(
        &self,
        cur_frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let command = VmBuiltins::extract_arg(cur_frame, |x| x.as_string().cloned())?;

        let output = Command::new(get_shell_path())
            .arg("-c")
            .arg(command.raw_value())
            .output();

        match output {
            Ok(output) => {
                let result = IntegerValue::from(output.status.code().unwrap_or(-1) as i64);
                result.write(
                    "stdout",
                    RuntimeValue::String(
                        String::from_utf8_lossy(&output.stdout).to_string().into(),
                    ),
                );
                result.write(
                    "stderr",
                    RuntimeValue::String(
                        String::from_utf8_lossy(&output.stderr).to_string().into(),
                    ),
                );
                cur_frame.stack.push(RuntimeValue::Integer(result));
                Ok(RunloopExit::Ok(()))
            }
            Err(e) => Err(VmErrorReason::OperationFailed(e.to_string()).into()),
        }
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "system"
    }
}

pub(super) fn insert_builtins(builtins: &mut VmBuiltins) {
    builtins.insert_builtin::<System>();
}
