// SPDX-License-Identifier: Apache-2.0
use crate::{
    builder::compiler_opcodes::CompilerOpcode,
    do_compile::{CompilationResult, CompileNode, CompileParams},
};

impl<'a> CompileNode<'a> for aria_parser::ast::AddOperation {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        self.left.do_compile(params)?;
        for right in &self.right {
            right.1.do_compile(params)?;
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(
                    match right.0 {
                        aria_parser::ast::AddSymbol::Plus => CompilerOpcode::Add,
                        aria_parser::ast::AddSymbol::Minus => CompilerOpcode::Sub,
                    },
                    self.loc.clone(),
                );
        }
        Ok(())
    }
}
