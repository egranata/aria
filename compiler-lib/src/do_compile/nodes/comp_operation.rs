// SPDX-License-Identifier: Apache-2.0
use crate::{
    builder::compiler_opcodes::CompilerOpcode,
    do_compile::{CompilationResult, CompileNode, CompileParams},
};

impl<'a> CompileNode<'a> for aria_parser::ast::CompOperation {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        self.left.do_compile(params)?;
        if let Some(right) = &self.right {
            right.1.do_compile(params)?;
            match right.0 {
                aria_parser::ast::CompSymbol::Equal => params
                    .writer
                    .get_current_block()
                    .write_opcode_and_source_info(CompilerOpcode::Equal, self.loc.clone()),
                aria_parser::ast::CompSymbol::NotEqual => params
                    .writer
                    .get_current_block()
                    .write_opcode_and_source_info(CompilerOpcode::Equal, self.loc.clone())
                    .write_opcode_and_source_info(CompilerOpcode::Not, self.loc.clone()),
                aria_parser::ast::CompSymbol::Isa => params
                    .writer
                    .get_current_block()
                    .write_opcode_and_source_info(CompilerOpcode::Isa, self.loc.clone()),
            };
        }
        Ok(())
    }
}
