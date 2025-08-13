// SPDX-License-Identifier: Apache-2.0
use haxby_opcodes::function_attribs::{FUNC_IS_METHOD, METHOD_ATTRIBUTE_TYPE};

use crate::{
    error::vm_error::VmErrorReason,
    frame::Frame,
    runtime_value::{
        RuntimeValue, builtin_type::BuiltinType, function::BuiltinFunctionImpl,
        kind::RuntimeValueType, list::List,
    },
    vm::RunloopExit,
};

use super::VmBuiltins;

#[derive(Default)]
struct StringLen {}
impl BuiltinFunctionImpl for StringLen {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let the_value = frame.stack.pop();
        if let Some(s) = the_value.as_string() {
            let len = s.len() as i64;
            frame.stack.push(RuntimeValue::Integer(len.into()));
            Ok(RunloopExit::Ok(()))
        } else {
            Err(VmErrorReason::UnexpectedType.into())
        }
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity::required(1)
    }

    fn name(&self) -> &str {
        "len"
    }
}

#[derive(Default)]
struct StringHasPrefix {}
impl BuiltinFunctionImpl for StringHasPrefix {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let this = match frame.stack.pop_if(|x| RuntimeValue::as_string(&x).cloned()) {
            Some(x) => x,
            None => {
                return Err(VmErrorReason::UnexpectedType.into());
            }
        };
        let prefix = match frame.stack.pop_if(|x| RuntimeValue::as_string(&x).cloned()) {
            Some(x) => x,
            None => {
                return Err(VmErrorReason::UnexpectedType.into());
            }
        };
        let result = this.raw_value().starts_with(&prefix.raw_value());
        frame.stack.push(RuntimeValue::Boolean(result.into()));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity::required(2)
    }

    fn name(&self) -> &str {
        "has_prefix"
    }
}

#[derive(Default)]
struct StringHasSuffix {}
impl BuiltinFunctionImpl for StringHasSuffix {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let this = match frame.stack.pop_if(|x| RuntimeValue::as_string(&x).cloned()) {
            Some(x) => x,
            None => {
                return Err(VmErrorReason::UnexpectedType.into());
            }
        };
        let prefix = match frame.stack.pop_if(|x| RuntimeValue::as_string(&x).cloned()) {
            Some(x) => x,
            None => {
                return Err(VmErrorReason::UnexpectedType.into());
            }
        };
        let result = this.raw_value().ends_with(&prefix.raw_value());
        frame.stack.push(RuntimeValue::Boolean(result.into()));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity::required(2)
    }

    fn name(&self) -> &str {
        "has_suffix"
    }
}

#[derive(Default)]
struct StringReplace {}
impl BuiltinFunctionImpl for StringReplace {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let this = match frame.stack.pop_if(|x| RuntimeValue::as_string(&x).cloned()) {
            Some(x) => x,
            None => {
                return Err(VmErrorReason::UnexpectedType.into());
            }
        };
        let current = match frame.stack.pop_if(|x| RuntimeValue::as_string(&x).cloned()) {
            Some(x) => x,
            None => {
                return Err(VmErrorReason::UnexpectedType.into());
            }
        };
        let wanted = match frame.stack.pop_if(|x| RuntimeValue::as_string(&x).cloned()) {
            Some(x) => x,
            None => {
                return Err(VmErrorReason::UnexpectedType.into());
            }
        };
        let result = this
            .raw_value()
            .replace(&current.raw_value(), &wanted.raw_value());
        frame.stack.push(RuntimeValue::String(result.into()));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity::required(3)
    }

    fn name(&self) -> &str {
        "replace"
    }
}

#[derive(Default)]
struct StringSplit {}
impl BuiltinFunctionImpl for StringSplit {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let this = match frame.stack.pop_if(|x| RuntimeValue::as_string(&x).cloned()) {
            Some(x) => x,
            None => {
                return Err(VmErrorReason::UnexpectedType.into());
            }
        };
        let marker = match frame.stack.pop_if(|x| RuntimeValue::as_string(&x).cloned()) {
            Some(x) => x,
            None => {
                return Err(VmErrorReason::UnexpectedType.into());
            }
        };
        let result = this
            .raw_value()
            .split(&marker.raw_value())
            .map(|x| RuntimeValue::String(x.to_owned().into()))
            .collect::<Vec<_>>();
        frame.stack.push(RuntimeValue::List(List::from(&result)));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity::required(2)
    }

    fn name(&self) -> &str {
        "split"
    }
}

#[derive(Default)]
struct StringChars {}
impl BuiltinFunctionImpl for StringChars {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let this = match frame.stack.pop_if(|x| RuntimeValue::as_string(&x).cloned()) {
            Some(x) => x.raw_value(),
            None => {
                return Err(VmErrorReason::UnexpectedType.into());
            }
        };

        let ret = List::default();
        this.chars()
            .map(|c| RuntimeValue::String(c.to_string().into()))
            .for_each(|rv| ret.append(rv));

        frame.stack.push(RuntimeValue::List(ret));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity::required(1)
    }

    fn name(&self) -> &str {
        "chars"
    }
}

#[derive(Default)]
struct StringBytes {}
impl BuiltinFunctionImpl for StringBytes {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let this = match frame.stack.pop_if(|x| RuntimeValue::as_string(&x).cloned()) {
            Some(x) => x.raw_value(),
            None => {
                return Err(VmErrorReason::UnexpectedType.into());
            }
        };

        let ret = List::default();
        this.bytes()
            .map(|c| RuntimeValue::Integer((c as i64).into()))
            .for_each(|rv| ret.append(rv));

        frame.stack.push(RuntimeValue::List(ret));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity::required(1)
    }

    fn name(&self) -> &str {
        "bytes"
    }
}

#[derive(Default)]
struct FromBytes {}
impl BuiltinFunctionImpl for FromBytes {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let this_str_type = match frame
            .stack
            .pop_if(|x| RuntimeValue::as_builtin_type(&x).clone())
        {
            Some(x) => x,
            None => {
                return Err(VmErrorReason::UnexpectedType.into());
            }
        };
        let list = match frame.stack.pop_if(|x| RuntimeValue::as_list(&x).cloned()) {
            Some(x) => x,
            None => {
                return Err(VmErrorReason::UnexpectedType.into());
            }
        };
        let mut bytes = vec![];
        for i in 0..list.len() {
            let item = list.get_at(i).expect("invalid list");
            if let Some(byte) = item.as_integer() {
                bytes.push(byte.raw_value() as u8);
            } else {
                return Err(VmErrorReason::UnexpectedType.into());
            }
        }
        let dest = match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => {
                let encoding_err_type = this_str_type
                    .read("EncodingError")
                    .expect("EncodingError not found")
                    .as_struct()
                    .expect("EncodingError not a struct");
                return Ok(RunloopExit::throw_struct(
                    &encoding_err_type,
                    &[("msg", RuntimeValue::String("invalid utf8".into()))],
                ));
            }
        };

        frame.stack.push(RuntimeValue::String(dest.into()));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD | METHOD_ATTRIBUTE_TYPE
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity::required(2)
    }

    fn name(&self) -> &str {
        "new_with_bytes"
    }
}

#[derive(Default)]
struct ToNumericEncoding {}
impl BuiltinFunctionImpl for ToNumericEncoding {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let this = match frame.stack.pop_if(|x| RuntimeValue::as_string(&x).cloned()) {
            Some(x) => x.raw_value(),
            None => {
                return Err(VmErrorReason::UnexpectedType.into());
            }
        };

        if let Some(char) = this.chars().next() {
            let char = char as i64;
            frame.stack.push(RuntimeValue::Integer(char.into()));
            Ok(RunloopExit::Ok(()))
        } else {
            Err(VmErrorReason::UnexpectedType.into())
        }
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity::required(1)
    }

    fn name(&self) -> &str {
        "encoding"
    }
}

#[derive(Default)]
struct TrimHead {}
impl BuiltinFunctionImpl for TrimHead {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let this = match frame.stack.pop_if(|x| RuntimeValue::as_string(&x).cloned()) {
            Some(x) => x.raw_value(),
            None => {
                return Err(VmErrorReason::UnexpectedType.into());
            }
        };

        let result = this.trim_start().to_string();
        frame.stack.push(RuntimeValue::String(result.into()));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity::required(1)
    }

    fn name(&self) -> &str {
        "trim_head"
    }
}

#[derive(Default)]
struct TrimTail {}
impl BuiltinFunctionImpl for TrimTail {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let this = match frame.stack.pop_if(|x| RuntimeValue::as_string(&x).cloned()) {
            Some(x) => x.raw_value(),
            None => {
                return Err(VmErrorReason::UnexpectedType.into());
            }
        };

        let result = this.trim_end().to_string();
        frame.stack.push(RuntimeValue::String(result.into()));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity::required(1)
    }

    fn name(&self) -> &str {
        "trim_tail"
    }
}

#[derive(Default)]
struct Uppercase {}
impl BuiltinFunctionImpl for Uppercase {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let this = match frame.stack.pop_if(|x| RuntimeValue::as_string(&x).cloned()) {
            Some(x) => x.raw_value(),
            None => {
                return Err(VmErrorReason::UnexpectedType.into());
            }
        };

        let result = this.to_uppercase();
        frame.stack.push(RuntimeValue::String(result.into()));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity::required(1)
    }

    fn name(&self) -> &str {
        "uppercase"
    }
}

#[derive(Default)]
struct Lowercase {}
impl BuiltinFunctionImpl for Lowercase {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let this = match frame.stack.pop_if(|x| RuntimeValue::as_string(&x).cloned()) {
            Some(x) => x.raw_value(),
            None => {
                return Err(VmErrorReason::UnexpectedType.into());
            }
        };

        let result = this.to_lowercase();
        frame.stack.push(RuntimeValue::String(result.into()));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity::required(1)
    }

    fn name(&self) -> &str {
        "lowercase"
    }
}

#[derive(Default)]
struct Contains {}
impl BuiltinFunctionImpl for Contains {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let this = VmBuiltins::extract_arg(frame, |a| a.as_string().cloned())?;
        let that = VmBuiltins::extract_arg(frame, |a| a.as_string().cloned())?;

        let contains = this.raw_value().contains(&that.raw_value());

        frame.stack.push(RuntimeValue::Boolean(contains.into()));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn arity(&self) -> crate::arity::Arity {
        crate::arity::Arity::required(2)
    }

    fn name(&self) -> &str {
        "contains"
    }
}

pub(super) fn insert_string_builtins(builtins: &mut VmBuiltins) {
    let string_builtin =
        BuiltinType::new(crate::runtime_value::builtin_type::BuiltinValueKind::String);

    string_builtin.insert_builtin::<StringLen>();
    string_builtin.insert_builtin::<StringHasPrefix>();
    string_builtin.insert_builtin::<StringHasSuffix>();
    string_builtin.insert_builtin::<StringReplace>();
    string_builtin.insert_builtin::<StringSplit>();
    string_builtin.insert_builtin::<StringChars>();
    string_builtin.insert_builtin::<StringBytes>();
    string_builtin.insert_builtin::<ToNumericEncoding>();
    string_builtin.insert_builtin::<FromBytes>();
    string_builtin.insert_builtin::<TrimHead>();
    string_builtin.insert_builtin::<TrimTail>();
    string_builtin.insert_builtin::<Uppercase>();
    string_builtin.insert_builtin::<Lowercase>();
    string_builtin.insert_builtin::<Contains>();

    builtins.insert(
        "String",
        RuntimeValue::Type(RuntimeValueType::Builtin(string_builtin)),
    );
}
