// SPDX-License-Identifier: Apache-2.0
use crate::{
    builder::compiler_opcodes::CompilerOpcode,
    do_compile::{CompilationResult, CompileNode, CompileParams},
};

impl<'a> CompileNode<'a> for aria_parser::ast::ShiftOperation {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        self.left.do_compile(params)?;
        if let Some(rhs) = &self.right {
            rhs.1.do_compile(params)?;
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(
                    match rhs.0 {
                        aria_parser::ast::ShiftSymbol::Leftward => CompilerOpcode::ShiftLeft,
                        aria_parser::ast::ShiftSymbol::Rightward => CompilerOpcode::ShiftRight,
                    },
                    self.loc.clone(),
                );
        };
        Ok(())
    }
}
