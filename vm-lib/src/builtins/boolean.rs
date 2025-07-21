// SPDX-License-Identifier: Apache-2.0
use crate::runtime_value::{builtin_type::BuiltinType, kind::RuntimeValueType, RuntimeValue};

use super::VmBuiltins;

pub(super) fn insert_boolean_builtins(builtins: &mut VmBuiltins) {
    let bool_builtin =
        BuiltinType::new(crate::runtime_value::builtin_type::BuiltinValueKind::Boolean);

    builtins.insert(
        "Bool",
        RuntimeValue::Type(RuntimeValueType::Builtin(bool_builtin)),
    );

    builtins.insert("true", RuntimeValue::Boolean(true.into()));
    builtins.insert("false", RuntimeValue::Boolean(false.into()));
}
