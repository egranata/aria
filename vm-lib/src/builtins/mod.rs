// SPDX-License-Identifier: Apache-2.0
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use haxby_opcodes::builtin_type_ids::*;

use crate::{
    error::vm_error::VmErrorReason,
    frame::Frame,
    runtime_value::{
        RuntimeValue,
        function::{BuiltinFunctionImpl, Function},
        kind::RuntimeValueType,
    },
};

mod alloc;
mod boolean;
mod cmdline_args;
mod float;
mod getenv;
mod hasattr;
mod identity;
mod integer;
mod list;
mod listattrs;
mod now;
mod prettyprint;
mod print;
mod println;
mod readattr;
mod readln;
mod string;
mod system;
mod timezone;
mod typeof_builtin;
mod writeattr;

pub struct VmBuiltins {
    values: Rc<RefCell<HashMap<String, RuntimeValue>>>,
}

impl VmBuiltins {
    pub fn insert_builtin<T>(&mut self)
    where
        T: 'static + Default + BuiltinFunctionImpl,
    {
        let t = T::default();
        let name = t.name().to_owned();
        self.insert(&name, RuntimeValue::Function(Function::builtin_from(t)));
    }

    pub fn extract_arg<T, U>(frame: &mut Frame, f: T) -> crate::vm::ExecutionResult<U>
    where
        T: FnOnce(RuntimeValue) -> Option<U>,
    {
        let val = match frame.stack.try_pop() {
            Some(v) => v,
            None => {
                return Err(VmErrorReason::EmptyStack.into());
            }
        };

        match f(val) {
            Some(v) => Ok(v),
            None => Err(VmErrorReason::UnexpectedType.into()),
        }
    }
}

impl Default for VmBuiltins {
    fn default() -> Self {
        let mut this = Self {
            values: Default::default(),
        };

        alloc::insert_builtins(&mut this);
        boolean::insert_boolean_builtins(&mut this);
        cmdline_args::insert_builtins(&mut this);
        integer::insert_integer_builtins(&mut this);
        float::insert_float_builtins(&mut this);
        getenv::insert_builtins(&mut this);
        hasattr::insert_builtins(&mut this);
        identity::insert_builtins(&mut this);
        list::insert_list_builtins(&mut this);
        listattrs::insert_builtins(&mut this);
        now::insert_builtins(&mut this);
        prettyprint::insert_builtins(&mut this);
        print::insert_builtins(&mut this);
        println::insert_builtins(&mut this);
        readln::insert_builtins(&mut this);
        string::insert_string_builtins(&mut this);
        readattr::insert_builtins(&mut this);
        system::insert_builtins(&mut this);
        timezone::insert_builtins(&mut this);
        typeof_builtin::insert_builtins(&mut this);
        writeattr::insert_builtins(&mut this);

        this.insert("Any", RuntimeValue::Type(RuntimeValueType::Any));
        this.insert("Module", RuntimeValue::Type(RuntimeValueType::Module));

        this
    }
}

impl VmBuiltins {
    pub fn load_named_value(&self, name: &str) -> Option<RuntimeValue> {
        self.values.borrow().get(name).cloned()
    }

    pub fn insert(&self, name: &str, val: RuntimeValue) {
        if self.values.borrow().contains_key(name) {
            panic!("duplicate builtin {name}");
        }

        self.values.borrow_mut().insert(name.to_owned(), val);
    }

    pub fn get_builtin_type_by_name(&self, name: &str) -> RuntimeValueType {
        self.load_named_value(name)
            .unwrap_or_else(|| panic!("missing {name} type"))
            .as_type()
            .unwrap_or_else(|| panic!("invalid {name} type"))
            .clone()
    }

    pub fn get_builtin_type_by_id(&self, n: u8) -> Option<RuntimeValueType> {
        match n {
            BUILTIN_TYPE_ANY => Some(self.get_builtin_type_by_name("Any")),
            BUILTIN_TYPE_INT => Some(self.get_builtin_type_by_name("Int")),
            BUILTIN_TYPE_FLOAT => Some(self.get_builtin_type_by_name("Float")),
            BUILTIN_TYPE_LIST => Some(self.get_builtin_type_by_name("List")),
            BUILTIN_TYPE_STRING => Some(self.get_builtin_type_by_name("String")),
            BUILTIN_TYPE_BOOL => Some(self.get_builtin_type_by_name("Bool")),
            BUILTIN_TYPE_MAYBE => Some(self.get_builtin_type_by_name("Maybe")),
            BUILTIN_TYPE_UNIMPLEMENTED => Some(self.get_builtin_type_by_name("Unimplemented")),
            BUILTIN_TYPE_RUNTIME_ERROR => Some(self.get_builtin_type_by_name("RuntimeError")),
            BUILTIN_TYPE_UNIT => Some(self.get_builtin_type_by_name("Unit")),
            _ => None,
        }
    }
}

impl VmBuiltins {
    pub fn create_maybe_some(&self, x: RuntimeValue) -> Result<RuntimeValue, VmErrorReason> {
        let rt_maybe = crate::some_or_err!(
            self.get_builtin_type_by_id(BUILTIN_TYPE_MAYBE),
            VmErrorReason::UnexpectedVmState
        );
        let rt_maybe_enum = crate::some_or_err!(rt_maybe.as_enum(), VmErrorReason::UnexpectedType);

        let some_idx = crate::some_or_err!(
            rt_maybe_enum.get_idx_of_case("Some"),
            VmErrorReason::NoSuchCase("Some".to_owned())
        );

        let rv = crate::some_or_err!(
            rt_maybe_enum.make_value(some_idx, Some(x)),
            VmErrorReason::UnexpectedVmState
        );

        Ok(RuntimeValue::EnumValue(rv))
    }

    pub fn create_maybe_none(&self) -> Result<RuntimeValue, VmErrorReason> {
        let rt_maybe = crate::some_or_err!(
            self.get_builtin_type_by_id(BUILTIN_TYPE_MAYBE),
            VmErrorReason::UnexpectedVmState
        );
        let rt_maybe_enum = crate::some_or_err!(rt_maybe.as_enum(), VmErrorReason::UnexpectedType);

        let none_idx = crate::some_or_err!(
            rt_maybe_enum.get_idx_of_case("None"),
            VmErrorReason::NoSuchCase("None".to_owned())
        );

        let rv = crate::some_or_err!(
            rt_maybe_enum.make_value(none_idx, None),
            VmErrorReason::UnexpectedVmState
        );

        Ok(RuntimeValue::EnumValue(rv))
    }

    pub fn create_unit_object(&self) -> Result<RuntimeValue, VmErrorReason> {
        let rt_unit = crate::some_or_err!(
            self.get_builtin_type_by_id(BUILTIN_TYPE_UNIT),
            VmErrorReason::UnexpectedVmState
        );
        let rt_unit_enum = crate::some_or_err!(rt_unit.as_enum(), VmErrorReason::UnexpectedType);

        let unit_idx = crate::some_or_err!(
            rt_unit_enum.get_idx_of_case("unit"),
            VmErrorReason::NoSuchCase("unit".to_owned())
        );

        let rv = crate::some_or_err!(
            rt_unit_enum.make_value(unit_idx, None),
            VmErrorReason::UnexpectedVmState
        );

        Ok(RuntimeValue::EnumValue(rv))
    }
}
