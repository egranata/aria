// SPDX-License-Identifier: Apache-2.0
use crate::builtins::VmBuiltins;
use crate::runtime_value::mixin::Mixin;
use crate::runtime_value::{RuntimeValue, RuntimeValueType};

#[derive(Clone)]
pub enum IsaCheckable {
    Type(RuntimeValueType),
    Mixin(Mixin),
}

impl IsaCheckable {
    pub fn isa_check(&self, other: &RuntimeValue, builtins: &VmBuiltins) -> bool {
        match self {
            IsaCheckable::Type(t) => other.isa(t, builtins),
            IsaCheckable::Mixin(m) => other.isa_mixin(m),
        }
    }

    pub fn any() -> Self {
        IsaCheckable::Type(RuntimeValueType::Any)
    }
}

impl TryFrom<&RuntimeValue> for IsaCheckable {
    type Error = ();

    fn try_from(value: &RuntimeValue) -> Result<Self, Self::Error> {
        if let Some(mixin) = value.as_mixin() {
            Ok(IsaCheckable::Mixin(mixin.clone()))
        } else if let Some(t) = value.as_type() {
            Ok(IsaCheckable::Type(t.clone()))
        } else {
            Err(())
        }
    }
}
