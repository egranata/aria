// SPDX-License-Identifier: Apache-2.0
use crate::{
    do_compile::{CompilationResult, CompileNode, CompileParams},
    func_builder::BasicBlockOpcode,
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
                        aria_parser::ast::ShiftSymbol::Leftward => BasicBlockOpcode::ShiftLeft,
                        aria_parser::ast::ShiftSymbol::Rightward => BasicBlockOpcode::ShiftRight,
                    },
                    self.loc.clone(),
                );
        };
        Ok(())
    }
}
