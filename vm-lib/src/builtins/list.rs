// SPDX-License-Identifier: Apache-2.0
use haxby_opcodes::function_attribs::FUNC_IS_METHOD;

use crate::{
    error::vm_error::VmErrorReason,
    frame::Frame,
    runtime_value::{
        RuntimeValue, builtin_type::BuiltinType, function::BuiltinFunctionImpl,
        kind::RuntimeValueType,
    },
    vm::RunloopExit,
};

use super::VmBuiltins;

#[derive(Default)]
struct ListLen {}
impl BuiltinFunctionImpl for ListLen {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let this = VmBuiltins::extract_arg(frame, |x| x.as_list().cloned())?;
        let len = this.len() as i64;
        frame.stack.push(RuntimeValue::Integer(len.into()));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "len"
    }
}

#[derive(Default)]
struct ListAppend {}
impl BuiltinFunctionImpl for ListAppend {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let this = VmBuiltins::extract_arg(frame, |x| x.as_list().cloned())?;
        let the_value = frame.stack.pop();
        this.append(the_value);
        frame.stack.push(RuntimeValue::List(this));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        2_u8
    }

    fn name(&self) -> &str {
        "append"
    }
}

#[derive(Default)]
struct Drop {}
impl BuiltinFunctionImpl for Drop {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let this = VmBuiltins::extract_arg(frame, |x| x.as_list().cloned())?;
        if this.is_empty() {
            Err(VmErrorReason::IndexOutOfBounds(0).into())
        } else {
            let the_value = this.get_at(this.len() - 1).unwrap();
            this.pop();
            frame.stack.push(the_value);
            Ok(RunloopExit::Ok(()))
        }
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "drop"
    }
}

pub(super) fn insert_list_builtins(builtins: &mut VmBuiltins) {
    let list_builtin = BuiltinType::new(crate::runtime_value::builtin_type::BuiltinValueKind::List);

    list_builtin.insert_builtin::<ListLen>();
    list_builtin.insert_builtin::<ListAppend>();
    list_builtin.insert_builtin::<Drop>();

    builtins.insert(
        "List",
        RuntimeValue::Type(RuntimeValueType::Builtin(list_builtin)),
    );
}
