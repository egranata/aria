// SPDX-License-Identifier: Apache-2.0

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::runtime_value::function::{BuiltinFunctionImpl, Function};

use super::{
    RuntimeValue,
    enum_case::{EnumValue, EnumValueImpl},
    kind::RuntimeValueType,
    mixin::Mixin,
};

#[derive(Clone)]
pub struct EnumCase {
    pub name: String,
    pub payload_type: Option<RuntimeValueType>,
}

#[derive(Default)]
pub struct EnumImpl {
    name: String,
    cases: RefCell<Vec<EnumCase>>,
    entries: RefCell<HashMap<String, RuntimeValue>>,
    mixins: RefCell<crate::mixin_includer::MixinIncluder>,
}

impl EnumImpl {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            cases: Default::default(),
            entries: RefCell::new(HashMap::new()),
            mixins: RefCell::new(crate::mixin_includer::MixinIncluder::default()),
        }
    }

    pub fn add_case(&self, case: EnumCase) -> usize {
        let idx = self.cases.borrow().len();
        self.cases.borrow_mut().push(case);
        idx
    }

    fn get_case_by_idx(&self, idx: usize) -> Option<EnumCase> {
        let b = self.cases.borrow();
        b.get(idx).cloned()
    }

    fn get_idx_of_case(&self, name: &str) -> Option<usize> {
        let b = self.cases.borrow();
        for (idx, case) in b.iter().enumerate() {
            if case.name == name {
                return Some(idx);
            }
        }
        None
    }

    fn load_named_value(&self, name: &str) -> Option<RuntimeValue> {
        if let Some(nv) = self.entries.borrow().get(name) {
            Some(nv.clone())
        } else {
            self.mixins.borrow().load_named_value(name)
        }
    }

    fn store_named_value(&self, name: &str, val: RuntimeValue) {
        self.entries.borrow_mut().insert(name.to_owned(), val);
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

#[derive(Clone)]
pub struct Enum {
    imp: Rc<EnumImpl>,
}

impl Enum {
    pub fn new(name: &str) -> Self {
        Self {
            imp: Rc::new(EnumImpl::new(name)),
        }
    }

    pub fn name(&self) -> &str {
        &self.imp.name
    }

    pub fn add_case(&self, case: EnumCase) -> usize {
        self.imp.add_case(case)
    }

    pub fn get_idx_of_case(&self, name: &str) -> Option<usize> {
        self.imp.get_idx_of_case(name)
    }

    pub fn get_case_by_idx(&self, idx: usize) -> Option<EnumCase> {
        self.imp.get_case_by_idx(idx)
    }

    pub fn load_named_value(&self, name: &str) -> Option<RuntimeValue> {
        self.imp.load_named_value(name)
    }

    pub fn store_named_value(&self, name: &str, val: RuntimeValue) {
        self.imp.store_named_value(name, val);
    }

    pub fn include_mixin(&self, mixin: &Mixin) {
        self.imp.include_mixin(mixin);
    }

    pub fn isa_mixin(&self, mixin: &Mixin) -> bool {
        self.imp.isa_mixin(mixin)
    }

    pub fn make_value(&self, cidx: usize, payload: Option<RuntimeValue>) -> Option<EnumValue> {
        match self.get_case_by_idx(cidx) {
            Some(case) => {
                if case.payload_type.is_some() == payload.is_some() {
                    Some(EnumValue {
                        imp: Rc::new(EnumValueImpl {
                            enumm: self.clone(),
                            case: cidx,
                            payload,
                        }),
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn list_attributes(&self) -> HashSet<String> {
        self.imp.list_attributes()
    }

    pub fn insert_builtin<T>(&self)
    where
        T: 'static + Default + BuiltinFunctionImpl,
    {
        let t = T::default();
        let name = t.name().to_owned();
        self.store_named_value(&name, RuntimeValue::Function(Function::builtin_from(t)));
    }
}

impl PartialEq for Enum {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.imp, &other.imp)
    }
}
impl Eq for Enum {}
