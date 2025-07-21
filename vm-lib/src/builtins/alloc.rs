// SPDX-License-Identifier: Apache-2.0
use crate::{
    builtins::VmBuiltins,
    error::vm_error::VmErrorReason,
    frame::Frame,
    runtime_value::{
        function::BuiltinFunctionImpl, kind::RuntimeValueType, object::Object, RuntimeValue,
    },
    vm::RunloopExit,
};

#[derive(Default)]
struct Alloc {}
impl BuiltinFunctionImpl for Alloc {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let alloc_type = VmBuiltins::extract_arg(frame, |x| x.as_type().cloned())?;

        match alloc_type {
            RuntimeValueType::Builtin(b) => {
                let rv = match b.get_tag() {
                    crate::runtime_value::builtin_type::BuiltinValueKind::Boolean => {
                        RuntimeValue::Boolean(false.into())
                    }
                    crate::runtime_value::builtin_type::BuiltinValueKind::Integer => {
                        RuntimeValue::Integer(0.into())
                    }
                    crate::runtime_value::builtin_type::BuiltinValueKind::Float => {
                        RuntimeValue::Float(0.0.into())
                    }
                    crate::runtime_value::builtin_type::BuiltinValueKind::List => {
                        RuntimeValue::List(crate::runtime_value::list::List::from(&[]))
                    }
                    crate::runtime_value::builtin_type::BuiltinValueKind::String => {
                        RuntimeValue::String("".into())
                    }
                };
                frame.stack.push(rv);
            }
            RuntimeValueType::Struct(s) => {
                let obj = RuntimeValue::Object(Object::new(&s));
                frame.stack.push(obj);
            }
            _ => {
                return Err(VmErrorReason::UnexpectedType.into());
            }
        }

        Ok(RunloopExit::Ok(()))
    }

    fn arity(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "alloc"
    }
}

pub(super) fn insert_builtins(builtins: &mut VmBuiltins) {
    builtins.insert_builtin::<Alloc>();
}
