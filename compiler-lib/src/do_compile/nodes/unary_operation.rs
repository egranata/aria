// SPDX-License-Identifier: Apache-2.0
use crate::{
    builder::compiler_opcodes::CompilerOpcode,
    do_compile::{CompilationResult, CompileNode, CompileParams},
};

impl<'a> CompileNode<'a> for aria_parser::ast::UnaryOperation {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        self.postfix.do_compile(params)?;
        if let Some(op) = self.operand {
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(
                    match op {
                        aria_parser::ast::UnarySymbol::Exclamation => CompilerOpcode::Not,
                        aria_parser::ast::UnarySymbol::Minus => CompilerOpcode::Neg,
                    },
                    self.loc.clone(),
                );
        }
        Ok(())
    }
}
