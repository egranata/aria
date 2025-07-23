// SPDX-License-Identifier: Apache-2.0

use aria_compiler::line_table::LineTable;
use aria_parser::ast::SourcePointer;

use crate::{
    runtime_value::{RuntimeValue, function::Function, kind::RuntimeValueType},
    stack::Stack,
    vm::VirtualMachine,
};

pub struct LocalVariable {
    pub val: RuntimeValue,
    pub ty: RuntimeValueType,
}

impl Default for LocalVariable {
    fn default() -> Self {
        Self {
            val: RuntimeValue::Integer(From::from(0)),
            ty: RuntimeValueType::Any,
        }
    }
}

#[derive(Clone)]
pub enum ControlBlock {
    Guard(RuntimeValue),
    Try(u16),
}

pub struct Frame {
    pub stack: Stack<RuntimeValue>,
    pub(crate) line_table: Option<LineTable>,
    pub(crate) ctrl_blocks: Stack<ControlBlock>,
    pub(crate) locals: Vec<LocalVariable>,
    pub(crate) func: Option<Function>,
}

impl Frame {
    pub(crate) fn drop_all_guards(&mut self, vm: &mut VirtualMachine) {
        while let Some(block) = self.ctrl_blocks.try_pop() {
            match block {
                ControlBlock::Guard(guard) => {
                    let _ = guard.eval(0, self, vm, true);
                }
                ControlBlock::Try(_) => {
                    // don't do anything here
                }
            }
        }
    }

    pub(crate) fn drop_to_first_try(&mut self, vm: &mut VirtualMachine) -> Option<u16> {
        while let Some(block) = self.ctrl_blocks.try_pop() {
            match block {
                ControlBlock::Guard(guard) => {
                    let _ = guard.eval(0, self, vm, true);
                }
                ControlBlock::Try(x) => {
                    return Some(x);
                }
            }
        }

        None
    }
}

impl Frame {
    pub fn new_with_function(f: Function) -> Self {
        let mut this = Self::new_with_n_locals(f.frame_size());
        this.set_line_table(f.line_table());
        this.func = Some(f);
        this
    }

    pub fn new_with_n_locals(n: u8) -> Self {
        let mut this = Self {
            stack: Default::default(),
            line_table: None,
            ctrl_blocks: Default::default(),
            locals: Vec::with_capacity(n as usize),
            func: None,
        };
        for _ in 0..n {
            this.locals.push(LocalVariable::default())
        }
        this
    }

    pub(crate) fn set_line_table(&mut self, lt: Option<&LineTable>) -> &mut Self {
        self.line_table = lt.cloned();
        self
    }

    pub(crate) fn get_line_table(&self) -> Option<&LineTable> {
        self.line_table.as_ref()
    }

    pub fn get_line_entry_at_pos(&self, pos: u16) -> Option<SourcePointer> {
        if let Some(lt) = &self.line_table {
            lt.get(pos)
        } else {
            None
        }
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self::new_with_n_locals(u8::MAX)
    }
}
