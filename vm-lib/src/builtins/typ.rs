// SPDX-License-Identifier: Apache-2.0
use crate::runtime_value::{RuntimeValue, builtin_type::BuiltinType, kind::RuntimeValueType};

use super::VmBuiltins;

pub(super) fn insert_type_builtins(builtins: &mut VmBuiltins) {
    let type_builtin = BuiltinType::new(crate::runtime_value::builtin_type::BuiltinValueKind::Type);

    builtins.insert(
        "Type",
        RuntimeValue::Type(RuntimeValueType::Builtin(type_builtin)),
    );
}
