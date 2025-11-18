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
    fn isa(val: &RuntimeValue, t: &RuntimeValueType, builtins: &VmBuiltins) -> bool {
        match t {
            RuntimeValueType::Any => true,
            RuntimeValueType::Union(u) => {
                for u_k in u {
                    if Self::isa(val, u_k, builtins) {
                        return true;
                    }
                }
                false
            }
            _ => RuntimeValueType::get_type(val, builtins) == *t,
        }
    }

    fn isa_mixin(val: &RuntimeValue, mixin: &Mixin) -> bool {
        if let Some(obj) = val.as_object() {
            obj.get_struct().isa_mixin(mixin)
        } else if let Some(env) = val.as_enum_value() {
            env.get_container_enum().isa_mixin(mixin)
        } else if let Some(m) = val.as_mixin() {
            m.isa_mixin(mixin)
        } else {
            match val.as_struct() {
                Some(st) => st.isa_mixin(mixin),
                _ => match val.as_enum() {
                    Some(en) => en.isa_mixin(mixin),
                    _ => false,
                },
            }
        }
    }
}

impl IsaCheckable {
    pub fn isa_check(&self, other: &RuntimeValue, builtins: &VmBuiltins) -> bool {
        match self {
            IsaCheckable::Type(t) => IsaCheckable::isa(other, t, builtins),
            IsaCheckable::Mixin(m) => IsaCheckable::isa_mixin(other, m),
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
