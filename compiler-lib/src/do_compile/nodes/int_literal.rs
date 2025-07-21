// SPDX-License-Identifier: Apache-2.0
use crate::{
    constant_value::ConstantValue,
    do_compile::{CompilationResult, CompileNode, CompileParams},
    func_builder::BasicBlockOpcode,
};

impl<'a> CompileNode<'a> for aria_parser::ast::IntLiteral {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        if self.val == 0 {
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(BasicBlockOpcode::Push0, self.loc.clone());
        } else if self.val == 1 {
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(BasicBlockOpcode::Push1, self.loc.clone());
        } else {
            let const_idx =
                self.insert_const_or_fail(params, ConstantValue::Integer(self.val), &self.loc)?;
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(BasicBlockOpcode::Push(const_idx), self.loc.clone());
        }
        Ok(())
    }
}
