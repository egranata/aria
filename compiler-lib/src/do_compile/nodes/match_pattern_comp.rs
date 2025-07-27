// SPDX-License-Identifier: Apache-2.0
use crate::{
    do_compile::{CompilationResult, CompileNode, CompileParams},
    func_builder::BasicBlockOpcode,
};

impl<'a> CompileNode<'a> for aria_parser::ast::MatchPatternComp {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        self.expr.do_compile(params)?;
        match self.op {
            aria_parser::ast::CompSymbol::Equal => {
                params
                    .writer
                    .get_current_block()
                    .write_opcode_and_source_info(BasicBlockOpcode::Equal, self.loc.clone());
            }
            aria_parser::ast::CompSymbol::NotEqual => {
                params
                    .writer
                    .get_current_block()
                    .write_opcode_and_source_info(BasicBlockOpcode::Equal, self.loc.clone())
                    .write_opcode_and_source_info(BasicBlockOpcode::Not, self.loc.clone());
            }
            aria_parser::ast::CompSymbol::Isa => {
                params
                    .writer
                    .get_current_block()
                    .write_opcode_and_source_info(BasicBlockOpcode::Isa, self.loc.clone());
            }
        }

        Ok(())
    }
}
