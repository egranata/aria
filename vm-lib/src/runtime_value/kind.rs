// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use enum_as_inner::EnumAsInner;

use crate::builtins::VmBuiltins;

use super::{
    AttributeError, RuntimeValue, builtin_type::BuiltinType, enumeration::Enum, structure::Struct,
};

#[derive(Clone, PartialEq, Eq)]
pub struct FunctionType {
    pub required_argc: u8,
    pub default_argc: u8,
    pub varargs: bool,
}

impl std::fmt::Debug for FunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}{})",
            self.required_argc,
            self.default_argc,
            if self.varargs { ", ..." } else { "" }
        )
    }
}

#[derive(EnumAsInner, Clone)]
pub enum RuntimeValueType {
    Any,
    Builtin(BuiltinType),
    CodeObject,
    Module,
    Function(FunctionType),
    BoundFunction(FunctionType),
    Mixin,
    Opaque,
    Struct(Struct),
    Enum(Enum),
    Type(Box<RuntimeValueType>),
    Union(Vec<RuntimeValueType>),
}

impl PartialEq for RuntimeValueType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Function(f0), Self::Function(f1)) => f0 == f1,
            (Self::BoundFunction(f0), Self::BoundFunction(f1)) => f0 == f1,
            (Self::Builtin(l0), Self::Builtin(r0)) => l0 == r0,
            (Self::Struct(l0), Self::Struct(r0)) => l0 == r0,
            (Self::Enum(l0), Self::Enum(r0)) => l0 == r0,
            (Self::Type(l0), Self::Type(r0)) => l0 == r0,
            (Self::Union(l0), Self::Union(r0)) => {
                if l0.len() != r0.len() {
                    false
                } else {
                    for vk_a in l0 {
                        if !r0.contains(vk_a) {
                            return false;
                        }
                    }
                    true
                }
            }
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
impl Eq for RuntimeValueType {}

impl RuntimeValueType {
    pub fn get_type(value: &RuntimeValue, builtins: &VmBuiltins) -> Self {
        match value {
            RuntimeValue::Object(obj) => Self::Struct(obj.get_struct().clone()),
            RuntimeValue::EnumValue(env) => Self::Enum(env.get_container_enum().clone()),
            RuntimeValue::CodeObject(_) => Self::CodeObject,
            RuntimeValue::Module(_) => Self::Module,
            RuntimeValue::Mixin(_) => Self::Mixin,
            RuntimeValue::Opaque(_) => Self::Opaque,
            RuntimeValue::Function(f) => Self::Function(FunctionType {
                required_argc: f.required_argc(),
                default_argc: f.default_argc(),
                varargs: f.varargs(),
            }),
            RuntimeValue::BoundFunction(bf) => Self::Function(FunctionType {
                required_argc: bf.func().required_argc(),
                default_argc: bf.func().default_argc(),
                varargs: bf.func().varargs(),
            }),
            RuntimeValue::Type(t) => Self::Type(Box::new(t.clone())),
            RuntimeValue::Boolean(_) => builtins.get_builtin_type_by_name("Bool"),
            RuntimeValue::Integer(_) => builtins.get_builtin_type_by_name("Int"),
            RuntimeValue::Float(_) => builtins.get_builtin_type_by_name("Float"),
            RuntimeValue::List(_) => builtins.get_builtin_type_by_name("List"),
            RuntimeValue::String(_) => builtins.get_builtin_type_by_name("String"),
        }
    }
}

impl std::fmt::Debug for RuntimeValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any => write!(f, "Any"),
            Self::Builtin(b) => write!(f, "{b:?}"),
            Self::CodeObject => write!(f, "CodeObject"),
            Self::Module => write!(f, "Module"),
            Self::Mixin => write!(f, "Mixin"),
            Self::Opaque => write!(f, "Opaque"),
            Self::Function(ft) => write!(f, "Function{ft:?}"),
            Self::BoundFunction(ft) => write!(f, "BoundFunction{ft:?}"),
            Self::Struct(s) => write!(f, "Struct {}", s.name()),
            Self::Enum(e) => write!(f, "Enum {}", e.name()),
            Self::Union(v) => {
                let us = v
                    .iter()
                    .map(|x| format!("{x:?}"))
                    .collect::<Vec<_>>()
                    .join("|");
                write!(f, "{us}")
            }
            Self::Type(t) => write!(f, "<type:{t:?}>"),
        }
    }
}

impl std::ops::BitOr<&RuntimeValueType> for &RuntimeValueType {
    type Output = RuntimeValueType;

    fn bitor(self, rhs: &RuntimeValueType) -> Self::Output {
        if *self == *rhs {
            return self.clone();
        }

        match (self, rhs) {
            (RuntimeValueType::Any, _) | (_, RuntimeValueType::Any) => RuntimeValueType::Any,
            (RuntimeValueType::Union(a), RuntimeValueType::Union(b)) => {
                let mut uret = a.clone();
                for b_vk in b {
                    if !uret.contains(b_vk) {
                        uret.push(b_vk.clone())
                    }
                }
                RuntimeValueType::Union(uret)
            }
            (RuntimeValueType::Union(a), _) => {
                if a.contains(rhs) {
                    self.clone()
                } else {
                    let mut uret = a.clone();
                    uret.push(rhs.clone());
                    RuntimeValueType::Union(uret)
                }
            }
            (_, RuntimeValueType::Union(b)) => {
                if b.contains(self) {
                    rhs.clone()
                } else {
                    let mut uret = b.clone();
                    uret.push(self.clone());
                    RuntimeValueType::Union(uret)
                }
            }
            _ => RuntimeValueType::Union(vec![self.clone(), rhs.clone()]),
        }
    }
}

impl RuntimeValueType {
    pub fn read_attribute(&self, attr_name: &str) -> Result<RuntimeValue, AttributeError> {
        if let Some(struk) = self.as_struct() {
            match struk.load_named_value(attr_name) {
                Some(x) => Ok(x),
                None => Err(AttributeError::NoSuchAttribute),
            }
        } else if let Some(enumm) = self.as_enum() {
            match enumm.load_named_value(attr_name) {
                Some(x) => Ok(x),
                None => Err(AttributeError::NoSuchAttribute),
            }
        } else if let Some(bt) = self.as_builtin() {
            match bt.read(attr_name) {
                Some(x) => Ok(x),
                None => Err(AttributeError::NoSuchAttribute),
            }
        } else {
            Err(AttributeError::ValueHasNoAttributes)
        }
    }

    pub fn write_attribute(
        &self,
        attr_name: &str,
        val: RuntimeValue,
    ) -> Result<(), AttributeError> {
        if let Some(struk) = self.as_struct() {
            struk.store_named_value(attr_name, val);
            Ok(())
        } else if let Some(enumm) = self.as_enum() {
            enumm.store_named_value(attr_name, val);
            Ok(())
        } else if let Some(bt) = self.as_builtin() {
            bt.write(attr_name, val);
            Ok(())
        } else {
            Err(AttributeError::ValueHasNoAttributes)
        }
    }

    pub fn list_attributes(&self) -> HashSet<String> {
        if let Some(struk) = self.as_struct() {
            struk.list_attributes()
        } else if let Some(enumm) = self.as_enum() {
            enumm.list_attributes()
        } else if let Some(bt) = self.as_builtin() {
            bt.list_attributes()
        } else {
            HashSet::new()
        }
    }
}
