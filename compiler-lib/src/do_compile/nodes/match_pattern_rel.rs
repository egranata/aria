// SPDX-License-Identifier: Apache-2.0
use crate::{
    builder::compiler_opcodes::CompilerOpcode,
    do_compile::{CompilationResult, CompileNode, CompileParams},
};

impl<'a> CompileNode<'a> for aria_parser::ast::MatchPatternRel {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        self.expr.do_compile(params)?;
        match self.op {
            aria_parser::ast::RelSymbol::Less => {
                params
                    .writer
                    .get_current_block()
                    .write_opcode_and_source_info(CompilerOpcode::LessThan, self.loc.clone());
            }
            aria_parser::ast::RelSymbol::LessEqual => {
                params
                    .writer
                    .get_current_block()
                    .write_opcode_and_source_info(CompilerOpcode::LessThanEqual, self.loc.clone());
            }
            aria_parser::ast::RelSymbol::Greater => {
                params
                    .writer
                    .get_current_block()
                    .write_opcode_and_source_info(CompilerOpcode::GreaterThan, self.loc.clone());
            }
            aria_parser::ast::RelSymbol::GreaterEqual => {
                params
                    .writer
                    .get_current_block()
                    .write_opcode_and_source_info(
                        CompilerOpcode::GreaterThanEqual,
                        self.loc.clone(),
                    );
            }
        }

        Ok(())
    }
}
