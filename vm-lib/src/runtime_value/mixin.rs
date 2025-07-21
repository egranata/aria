// SPDX-License-Identifier: Apache-2.0
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use super::RuntimeValue;

#[derive(Default)]
struct MixinImpl {
    entries: RefCell<HashMap<String, RuntimeValue>>,
    mixins: RefCell<crate::mixin_includer::MixinIncluder>,
}

impl MixinImpl {
    fn load_named_value(&self, name: &str) -> Option<RuntimeValue> {
        if let Some(val) = self.entries.borrow().get(name) {
            Some(val.clone())
        } else {
            self.mixins.borrow().load_named_value(name)
        }
    }

    fn store_named_value(&self, name: &str, val: RuntimeValue) {
        self.entries.borrow_mut().insert(name.to_owned(), val);
    }

    fn named_values(&self) -> Vec<String> {
        self.entries.borrow().keys().cloned().collect()
    }

    fn include_mixin(&self, mixin: &Mixin) {
        self.mixins.borrow_mut().include(mixin.clone());
    }

    fn isa_mixin(&self, mixin: &Mixin) -> bool {
        self.mixins.borrow().contains(mixin)
    }

    fn list_attributes(&self) -> HashSet<String> {
        let mut attrs: HashSet<String> = self.entries.borrow().keys().cloned().collect();
        attrs.extend(self.mixins.borrow().list_attributes());
        attrs
    }
}

#[derive(Default, Clone)]
pub struct Mixin {
    imp: Rc<MixinImpl>,
}

impl Mixin {
    pub fn load_named_value(&self, name: &str) -> Option<RuntimeValue> {
        self.imp.load_named_value(name)
    }

    pub fn store_named_value(&self, name: &str, val: RuntimeValue) {
        self.imp.store_named_value(name, val);
    }

    pub fn named_values(&self) -> Vec<String> {
        self.imp.named_values()
    }

    pub fn identity(&self) -> usize {
        Rc::as_ptr(&self.imp) as usize
    }

    pub fn include_mixin(&self, mixin: &Mixin) {
        self.imp.include_mixin(mixin);
    }

    pub fn isa_mixin(&self, mixin: &Mixin) -> bool {
        self == mixin || self.imp.isa_mixin(mixin)
    }

    pub fn list_attributes(&self) -> HashSet<String> {
        self.imp.list_attributes()
    }
}

impl PartialEq for Mixin {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.imp, &other.imp)
    }
}
impl Eq for Mixin {}
