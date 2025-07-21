// SPDX-License-Identifier: Apache-2.0
use std::rc::Rc;

use aria_compiler::{constant_value::CompiledCodeObject, line_table::LineTable};
use aria_parser::ast::SourcePointer;

#[derive(Clone)]
pub struct CodeObject {
    pub name: String,
    pub body: Rc<[u8]>,
    pub arity: u8,
    pub frame_size: u8,
    pub loc: SourcePointer,
    pub line_table: Rc<LineTable>,
}

impl PartialEq for CodeObject {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.body, &other.body)
    }
}

impl From<&CompiledCodeObject> for CodeObject {
    fn from(value: &CompiledCodeObject) -> Self {
        Self {
            name: value.name.clone(),
            body: Rc::from(value.body.as_slice()),
            arity: value.arity,
            frame_size: value.frame_size,
            loc: value.loc.clone(),
            line_table: Rc::from(value.line_table.clone()),
        }
    }
}

impl std::fmt::Debug for CodeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<code-object {} at {}>", self.name, self.loc)
    }
}

impl CodeObject {
    pub fn identity(&self) -> usize {
        let ptr = Rc::as_ptr(&self.body);
        ptr as *const () as usize
    }
}
