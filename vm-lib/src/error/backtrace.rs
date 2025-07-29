// SPDX-License-Identifier: Apache-2.0

use aria_parser::ast::SourcePointer;

#[derive(Clone, Debug, Default)]
pub struct Backtrace {
    entries: Vec<SourcePointer>,
}

impl Backtrace {
    pub fn first_entry(&self) -> Option<SourcePointer> {
        self.entries.first().cloned()
    }

    pub fn entries_iter(&self) -> std::slice::Iter<'_, SourcePointer> {
        self.entries.iter()
    }

    pub fn push(&mut self, loc: SourcePointer) {
        self.entries.push(loc);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
