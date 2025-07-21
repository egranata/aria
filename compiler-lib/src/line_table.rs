// SPDX-License-Identifier: Apache-2.0
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use aria_parser::ast::SourcePointer;

#[derive(Default)]
struct LineTableImpl {
    map: RefCell<HashMap<u16, SourcePointer>>,
}

#[derive(Clone, Default)]
pub struct LineTable {
    imp: Rc<LineTableImpl>,
}

impl LineTable {
    pub fn insert(&self, idx: u16, ptr: SourcePointer) {
        self.imp.map.borrow_mut().insert(idx, ptr);
    }

    pub fn get(&self, idx: u16) -> Option<SourcePointer> {
        self.imp.map.borrow().get(&idx).cloned()
    }
}

impl PartialEq for LineTable {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.imp, &other.imp)
    }
}
impl Eq for LineTable {}
