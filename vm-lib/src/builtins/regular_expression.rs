// SPDX-License-Identifier: Apache-2.0
use haxby_opcodes::function_attribs::{FUNC_IS_METHOD, METHOD_ATTRIBUTE_TYPE};

use crate::{
    error::{exception::VmException, vm_error::VmErrorReason},
    frame::Frame,
    runtime_value::{
        function::BuiltinFunctionImpl, kind::RuntimeValueType, list::List, object::Object,
        opaque::OpaqueValue, structure::Struct, RuntimeValue,
    },
    some_or_err,
    vm::RunloopExit,
};

use super::VmBuiltins;

fn create_regex_error(
    regex_struct: &Struct,
    message: String,
) -> Result<RuntimeValue, VmErrorReason> {
    let regex_error = some_or_err!(
        regex_struct.load_named_value("Error"),
        VmErrorReason::UnexpectedVmState
    );

    let regex_error = some_or_err!(regex_error.as_struct(), VmErrorReason::UnexpectedType);

    let regex_error = Object::new(&regex_error);
    regex_error.write("msg", RuntimeValue::String(message.into()));

    Ok(RuntimeValue::Object(regex_error))
}

#[derive(Default)]
struct New {}
impl BuiltinFunctionImpl for New {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let the_struct = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_struct().clone())?;
        let the_pattern = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_string().cloned())?;

        let rust_regex_obj = match regex::Regex::new(&the_pattern.raw_value()) {
            Ok(s) => s,
            Err(e) => {
                let err = create_regex_error(&the_struct, e.to_string());
                return match err {
                    Ok(s) => Ok(RunloopExit::Exception(VmException::from_value(s))),
                    Err(e) => Err(e.into()),
                };
            }
        };

        let rust_regex_obj = OpaqueValue::new(rust_regex_obj);

        let aria_regex_obj = Object::new(&the_struct);
        aria_regex_obj.write("__pattern", RuntimeValue::Opaque(rust_regex_obj));
        aria_regex_obj.write("pattern", RuntimeValue::String(the_pattern));

        frame.stack.push(RuntimeValue::Object(aria_regex_obj));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD | METHOD_ATTRIBUTE_TYPE
    }

    fn arity(&self) -> u8 {
        2_u8
    }

    fn name(&self) -> &str {
        "new"
    }
}

#[derive(Default)]
struct AnyMatch {}
impl BuiltinFunctionImpl for AnyMatch {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let aria_regex = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;
        let the_haystack =
            VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_string().cloned())?;

        let rust_regex_obj = match aria_regex.read("__pattern") {
            Some(s) => s,
            None => return Err(VmErrorReason::UnexpectedVmState.into()),
        };
        let rust_regex_obj = match rust_regex_obj.as_opaque_concrete::<regex::Regex>() {
            Some(s) => s,
            None => return Err(VmErrorReason::UnexpectedVmState.into()),
        };

        let matches = rust_regex_obj.is_match(&the_haystack.raw_value());

        frame.stack.push(RuntimeValue::Boolean(matches.into()));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn arity(&self) -> u8 {
        2_u8
    }

    fn name(&self) -> &str {
        "any_match"
    }
}

#[derive(Default)]
struct Matches {}
impl BuiltinFunctionImpl for Matches {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let aria_regex = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;
        let the_haystack =
            VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_string().cloned())?.raw_value();

        let match_struct_type = aria_regex
            .get_struct()
            .load_named_value("Match")
            .unwrap()
            .as_struct()
            .unwrap();

        let rust_regex_obj = match aria_regex.read("__pattern") {
            Some(s) => s,
            None => return Err(VmErrorReason::UnexpectedVmState.into()),
        };
        let rust_regex_obj = match rust_regex_obj.as_opaque_concrete::<regex::Regex>() {
            Some(s) => s,
            None => return Err(VmErrorReason::UnexpectedVmState.into()),
        };

        let matches: Vec<_> = rust_regex_obj
            .find_iter(&the_haystack)
            .map(|mh| (mh.start() as i64, mh.len() as i64, mh.as_str()))
            .collect();

        let matches_list = List::default();
        for m in matches {
            let match_obj = Object::new(&match_struct_type);
            match_obj.write("start", RuntimeValue::Integer(m.0.into()));
            match_obj.write("len", RuntimeValue::Integer(m.1.into()));
            match_obj.write("value", RuntimeValue::String(m.2.into()));
            matches_list.append(RuntimeValue::Object(match_obj));
        }

        frame.stack.push(RuntimeValue::List(matches_list));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn arity(&self) -> u8 {
        2_u8
    }

    fn name(&self) -> &str {
        "matches"
    }
}

#[derive(Default)]
struct Replace {}
impl BuiltinFunctionImpl for Replace {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let aria_regex = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;

        let the_haystack =
            VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_string().cloned())?.raw_value();

        let new_value =
            VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_string().cloned())?.raw_value();

        let rust_regex_obj = match aria_regex.read("__pattern") {
            Some(s) => s,
            None => return Err(VmErrorReason::UnexpectedVmState.into()),
        };
        let rust_regex_obj = match rust_regex_obj.as_opaque_concrete::<regex::Regex>() {
            Some(s) => s,
            None => return Err(VmErrorReason::UnexpectedVmState.into()),
        };

        let target = rust_regex_obj
            .replace_all(&the_haystack, new_value)
            .to_string();

        frame.stack.push(RuntimeValue::String(target.into()));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn arity(&self) -> u8 {
        3_u8
    }

    fn name(&self) -> &str {
        "replace"
    }
}

pub(super) fn insert_rgx_builtins(builtins: &mut VmBuiltins) {
    let regex_struct = Struct::new("Regex");

    regex_struct.store_named_value(
        "Match",
        RuntimeValue::Type(RuntimeValueType::Struct(Struct::new("Match"))),
    );

    regex_struct.insert_builtin::<New>();
    regex_struct.insert_builtin::<AnyMatch>();
    regex_struct.insert_builtin::<Matches>();
    regex_struct.insert_builtin::<Replace>();

    builtins.insert(
        "Regex",
        RuntimeValue::Type(RuntimeValueType::Struct(regex_struct)),
    );
}
