// SPDX-License-Identifier: Apache-2.0
use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::{
    error::vm_error::{VmError, VmErrorReason},
    frame::Frame,
    runtime_value::object::ObjectBox,
    vm::{ExecutionResult, VirtualMachine},
};

use super::RuntimeValue;

#[derive(Default)]
pub(super) struct ListImpl {
    values: RefCell<Vec<RuntimeValue>>,
    boxx: ObjectBox,
}

impl ListImpl {
    fn len(&self) -> usize {
        self.values.borrow().len()
    }

    fn is_empty(&self) -> bool {
        self.values.borrow().is_empty()
    }

    fn get_at(&self, idx: usize) -> Option<RuntimeValue> {
        self.values.borrow().get(idx).cloned()
    }

    fn append(&self, val: RuntimeValue) {
        self.values.borrow_mut().push(val)
    }

    fn pop(&self) {
        self.values.borrow_mut().pop();
    }

    fn set_at(&self, idx: usize, val: RuntimeValue) {
        match idx.cmp(&self.len()) {
            std::cmp::Ordering::Less => {
                self.values.borrow_mut()[idx] = val;
            }
            std::cmp::Ordering::Equal => {
                self.append(val);
            }
            std::cmp::Ordering::Greater => todo!(),
        }
    }

    fn write(&self, name: &str, val: RuntimeValue) {
        self.boxx.write(name, val)
    }

    fn read(&self, name: &str) -> Option<RuntimeValue> {
        self.boxx.read(name)
    }

    fn list_attributes(&self) -> HashSet<String> {
        self.boxx.list_attributes()
    }
}

impl std::fmt::Debug for ListImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let li = self.values.borrow();
        write!(
            f,
            "[{}]",
            li.iter()
                .map(|x| format!("{x:?}"))
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

#[derive(Clone, Default)]
pub struct List {
    pub(super) imp: Rc<ListImpl>,
}

impl List {
    pub fn from(values: &[RuntimeValue]) -> Self {
        let ret = Self::default();
        values.iter().cloned().for_each(|v| ret.append(v));
        ret
    }

    pub fn len(&self) -> usize {
        self.imp.len()
    }

    pub fn is_empty(&self) -> bool {
        self.imp.is_empty()
    }

    pub fn get_at(&self, idx: usize) -> Option<RuntimeValue> {
        self.imp.get_at(idx)
    }

    pub fn append(&self, val: RuntimeValue) {
        self.imp.append(val)
    }

    pub fn pop(&self) {
        self.imp.pop()
    }

    pub fn set_at(&self, idx: usize, val: RuntimeValue) {
        self.imp.set_at(idx, val)
    }

    pub fn read_index(
        &self,
        idx: &RuntimeValue,
        _: &mut Frame,
        _: &mut VirtualMachine,
    ) -> Result<RuntimeValue, VmError> {
        if let Some(i) = idx.as_integer() {
            if let Some(val) = self.get_at(i.raw_value() as usize) {
                Ok(val)
            } else {
                Err(VmErrorReason::IndexOutOfBounds(i.raw_value() as usize).into())
            }
        } else {
            Err(VmErrorReason::UnexpectedType.into())
        }
    }

    pub fn write_index(
        &self,
        idx: &RuntimeValue,
        val: &RuntimeValue,
        _: &mut Frame,
        _: &mut VirtualMachine,
    ) -> ExecutionResult {
        if let Some(i) = idx.as_integer() {
            self.set_at(i.raw_value() as usize, val.clone());
            Ok(())
        } else {
            Err(VmErrorReason::UnexpectedType.into())
        }
    }

    pub fn write(&self, name: &str, val: RuntimeValue) {
        self.imp.write(name, val)
    }

    pub fn read(&self, name: &str) -> Option<RuntimeValue> {
        self.imp.read(name)
    }

    pub fn identity(&self) -> usize {
        Rc::as_ptr(&self.imp) as usize
    }

    pub fn list_attributes(&self) -> HashSet<String> {
        self.imp.list_attributes()
    }
}

impl std::fmt::Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.imp)
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.imp, &other.imp)
    }
}
impl Eq for List {}
