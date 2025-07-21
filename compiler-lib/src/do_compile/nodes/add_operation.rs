// SPDX-License-Identifier: Apache-2.0
use crate::{
    do_compile::{CompilationResult, CompileNode, CompileParams},
    func_builder::BasicBlockOpcode,
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
                        aria_parser::ast::AddSymbol::Plus => BasicBlockOpcode::Add,
                        aria_parser::ast::AddSymbol::Minus => BasicBlockOpcode::Sub,
                    },
                    self.loc.clone(),
                );
        }
        Ok(())
    }
}
