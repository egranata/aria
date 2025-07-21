// SPDX-License-Identifier: Apache-2.0
use crate::constant_value::{CompiledCodeObject, ConstantValue, ConstantValues};

#[derive(Default)]
pub struct CompiledModule {
    pub constants: ConstantValues,
}

impl CompiledModule {
    pub fn load_indexed_const(&self, idx: u16) -> Option<ConstantValue> {
        self.constants.get(idx as usize)
    }

    // relies on __entry being the last code object stored in the module
    // after everything else is compiled
    pub fn load_entry_code_object(&self) -> CompiledCodeObject {
        let cco = self
            .constants
            .get(self.constants.len() - 1)
            .expect("missing __entry constant");

        cco.as_compiled_code_object()
            .expect("__entry constant is not a code object")
            .clone()
    }
}
