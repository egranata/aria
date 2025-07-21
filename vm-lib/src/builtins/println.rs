// SPDX-License-Identifier: Apache-2.0
use crate::{
    builtins::VmBuiltins, frame::Frame, runtime_value::function::BuiltinFunctionImpl,
    vm::RunloopExit,
};

#[derive(Default)]
struct Println {}
impl BuiltinFunctionImpl for Println {
    fn eval(
        &self,
        cur_frame: &mut Frame,
        vm: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let the_value = cur_frame.stack.pop();
        println!("{}", the_value.prettyprint(cur_frame, vm));
        Ok(RunloopExit::Ok(()))
    }

    fn arity(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "println"
    }
}

pub(super) fn insert_builtins(builtins: &mut VmBuiltins) {
    builtins.insert_builtin::<Println>();
}
