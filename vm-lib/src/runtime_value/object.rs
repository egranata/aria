// SPDX-License-Identifier: Apache-2.0
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::error::vm_error::VmErrorReason;

use super::{structure::Struct, RuntimeValue};

#[derive(Default)]
pub struct ObjectBox {
    values: RefCell<HashMap<String, RuntimeValue>>,
}

impl ObjectBox {
    pub fn write(&self, name: &str, val: RuntimeValue) {
        self.values.borrow_mut().insert(name.to_owned(), val);
    }

    pub fn read(&self, name: &str) -> Option<RuntimeValue> {
        self.values.borrow().get(name).cloned()
    }

    fn delete(&self, name: &str) {
        self.values.borrow_mut().remove(name);
    }

    pub(super) fn list_attributes(&self) -> HashSet<String> {
        self.values.borrow().keys().cloned().collect()
    }
}

struct ObjectImpl {
    boxx: ObjectBox,
    kind: Struct,
}

#[derive(Clone)]
pub struct Object {
    imp: Rc<ObjectImpl>,
}

impl ObjectImpl {
    fn new(kind: &Struct) -> Self {
        Self {
            boxx: Default::default(),
            kind: kind.clone(),
        }
    }

    fn write(&self, name: &str, val: RuntimeValue) {
        self.boxx.write(name, val)
    }

    fn read(&self, name: &str) -> Option<RuntimeValue> {
        self.boxx.read(name)
    }

    fn delete(&self, name: &str) {
        self.boxx.delete(name);
    }

    fn list_attributes(&self) -> HashSet<String> {
        self.boxx.list_attributes()
    }
}

impl Object {
    pub fn new(kind: &Struct) -> Self {
        Self {
            imp: Rc::new(ObjectImpl::new(kind)),
        }
    }

    pub fn write(&self, name: &str, val: RuntimeValue) {
        self.imp.write(name, val);
    }

    pub fn read(&self, name: &str) -> Option<RuntimeValue> {
        self.imp.read(name)
    }

    pub fn list_attributes(&self) -> HashSet<String> {
        self.imp.list_attributes()
    }

    pub(crate) fn delete(&self, name: &str) {
        self.imp.delete(name);
    }

    pub fn get_struct(&self) -> &Struct {
        &self.imp.kind
    }

    pub fn identity(&self) -> usize {
        Rc::as_ptr(&self.imp) as usize
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.imp, &other.imp)
    }
}
impl Eq for Object {}

impl Object {
    pub fn extract_field<FnType, OkType>(
        &self,
        name: &str,
        f: FnType,
    ) -> Result<OkType, VmErrorReason>
    where
        FnType: FnOnce(RuntimeValue) -> Option<OkType>,
    {
        let val = match self.read(name) {
            Some(v) => v,
            None => {
                return Err(VmErrorReason::NoSuchIdentifier(name.to_owned()));
            }
        };

        match f(val) {
            Some(v) => Ok(v),
            None => Err(VmErrorReason::UnexpectedType),
        }
    }
}
