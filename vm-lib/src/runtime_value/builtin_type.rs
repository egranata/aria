// SPDX-License-Identifier: Apache-2.0
use std::{cell::RefCell, rc::Rc};

use enum_as_inner::EnumAsInner;
use rustc_data_structures::fx::FxHashSet;

use super::{
    RuntimeValue,
    function::{BuiltinFunctionImpl, Function},
    mixin::Mixin,
    object::ObjectBox,
};

#[derive(EnumAsInner, Clone, PartialEq, Eq)]
pub enum BuiltinValueKind {
    Boolean,
    Integer,
    Float,
    List,
    String,
    Type,
}

struct BuiltinTypeImpl {
    tag: BuiltinValueKind,
    boxx: Rc<ObjectBox>,
    mixins: RefCell<crate::mixin_includer::MixinIncluder>,
}

impl BuiltinTypeImpl {
    fn write(&self, name: &str, val: RuntimeValue) {
        self.boxx.write(name, val)
    }

    fn read(&self, name: &str) -> Option<RuntimeValue> {
        match self.boxx.read(name) {
            Some(nv) => Some(nv),
            _ => self.mixins.borrow().load_named_value(name),
        }
    }

    fn include_mixin(&self, mixin: &Mixin) {
        self.mixins.borrow_mut().include(mixin.clone());
    }

    fn list_attributes(&self) -> FxHashSet<String> {
        let mut attrs = self.boxx.list_attributes();
        attrs.extend(self.mixins.borrow().list_attributes());
        attrs
    }
}

#[derive(Clone)]
pub struct BuiltinType {
    imp: Rc<BuiltinTypeImpl>,
}

impl BuiltinType {
    pub fn new(rvt: BuiltinValueKind) -> Self {
        Self {
            imp: Rc::new(BuiltinTypeImpl {
                tag: rvt,
                boxx: Rc::new(Default::default()),
                mixins: Default::default(),
            }),
        }
    }

    pub fn get_tag(&self) -> &BuiltinValueKind {
        &self.imp.tag
    }

    pub fn get_boxx(&self) -> &Rc<ObjectBox> {
        &self.imp.boxx
    }

    pub fn write(&self, name: &str, val: RuntimeValue) {
        self.imp.write(name, val);
    }

    pub fn read(&self, name: &str) -> Option<RuntimeValue> {
        self.imp.read(name)
    }

    pub fn include_mixin(&self, mixin: &Mixin) {
        self.imp.include_mixin(mixin);
    }

    pub fn insert_builtin<T>(&self)
    where
        T: 'static + Default + BuiltinFunctionImpl,
    {
        let t = T::default();
        let name = t.name().to_owned();
        self.get_boxx()
            .write(&name, RuntimeValue::Function(Function::builtin_from(t)));
    }

    pub fn list_attributes(&self) -> FxHashSet<String> {
        self.imp.list_attributes()
    }
}

impl PartialEq for BuiltinType {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.imp, &other.imp) || self.imp.tag == other.imp.tag
    }
}
impl Eq for BuiltinType {}

impl std::fmt::Debug for BuiltinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.get_tag() {
            BuiltinValueKind::Boolean => write!(f, "Bool"),
            BuiltinValueKind::Integer => write!(f, "Int"),
            BuiltinValueKind::Float => write!(f, "Float"),
            BuiltinValueKind::List => write!(f, "List"),
            BuiltinValueKind::String => write!(f, "String"),
            BuiltinValueKind::Type => write!(f, "Type"),
        }
    }
}
