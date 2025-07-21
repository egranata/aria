// SPDX-License-Identifier: Apache-2.0
use crate::{
    constant_value::ConstantValue,
    do_compile::{CompilationResult, CompileNode, CompileParams},
    func_builder::BasicBlockOpcode,
};

impl<'a> CompileNode<'a> for aria_parser::ast::ImportStatement {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        let path_idx = self.insert_const_or_fail(
            params,
            ConstantValue::String(self.what.to_dotted_string()),
            &self.loc,
        )?;
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(BasicBlockOpcode::Import(path_idx), self.loc.clone());
        Ok(())
    }
}
