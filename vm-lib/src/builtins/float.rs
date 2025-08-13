// SPDX-License-Identifier: Apache-2.0

use haxby_opcodes::function_attribs::FUNC_IS_METHOD;

use crate::{
    frame::Frame,
    runtime_value::{
        RuntimeValue, builtin_type::BuiltinType, function::BuiltinFunctionImpl,
        kind::RuntimeValueType,
    },
    vm::RunloopExit,
};

use super::VmBuiltins;

#[derive(Default)]
struct FpHash {}
impl BuiltinFunctionImpl for FpHash {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let this = VmBuiltins::extract_arg(frame, |x| x.as_float().cloned())?;
        let hv = unsafe { std::mem::transmute_copy::<f64, i64>(&this.raw_value()) };
        frame.stack.push(RuntimeValue::Integer(hv.into()));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity::required(1)
    }

    fn name(&self) -> &str {
        "hash"
    }
}

#[derive(Default)]
struct FpFloor {}
impl BuiltinFunctionImpl for FpFloor {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let this = VmBuiltins::extract_arg(frame, |x| x.as_float().cloned())?;
        let result = RuntimeValue::Float(this.raw_value().floor().into());
        frame.stack.push(result);
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity::required(1)
    }

    fn name(&self) -> &str {
        "floor"
    }
}

#[derive(Default)]
struct FpCeil {}
impl BuiltinFunctionImpl for FpCeil {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let this = VmBuiltins::extract_arg(frame, |x| x.as_float().cloned())?;
        let result = RuntimeValue::Float(this.raw_value().ceil().into());
        frame.stack.push(result);
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity::required(1)
    }

    fn name(&self) -> &str {
        "ceil"
    }
}

#[derive(Default)]
struct FpInt {}
impl BuiltinFunctionImpl for FpInt {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let this = VmBuiltins::extract_arg(frame, |x| x.as_float().cloned())?;
        let iv = this.raw_value() as i64;
        frame.stack.push(RuntimeValue::Integer(iv.into()));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity::required(1)
    }

    fn name(&self) -> &str {
        "int"
    }
}

pub(super) fn insert_float_builtins(builtins: &mut VmBuiltins) {
    let fp_builtin = BuiltinType::new(crate::runtime_value::builtin_type::BuiltinValueKind::Float);

    fp_builtin.insert_builtin::<FpHash>();
    fp_builtin.insert_builtin::<FpFloor>();
    fp_builtin.insert_builtin::<FpCeil>();
    fp_builtin.insert_builtin::<FpInt>();

    fp_builtin.write("inf", RuntimeValue::Float(f64::INFINITY.into()));
    fp_builtin.write("nan", RuntimeValue::Float(f64::NAN.into()));
    fp_builtin.write("epsilon", RuntimeValue::Float(f64::EPSILON.into()));

    builtins.insert(
        "Float",
        RuntimeValue::Type(RuntimeValueType::Builtin(fp_builtin)),
    );
}
