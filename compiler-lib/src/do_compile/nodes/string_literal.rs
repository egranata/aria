// SPDX-License-Identifier: Apache-2.0
use crate::{
    builder::compiler_opcodes::CompilerOpcode,
    constant_value::ConstantValue,
    do_compile::{CompilationResult, CompileNode, CompileParams},
};

impl<'a> CompileNode<'a> for aria_parser::ast::StringLiteral {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        let const_idx = self.insert_const_or_fail(
            params,
            ConstantValue::String(self.value.clone()),
            &self.loc,
        )?;
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(CompilerOpcode::Push(const_idx), self.loc.clone());
        Ok(())
    }
}
