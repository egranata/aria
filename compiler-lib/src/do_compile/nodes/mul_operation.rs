// SPDX-License-Identifier: Apache-2.0
use crate::{
    do_compile::{CompilationResult, CompileNode, CompileParams},
    func_builder::BasicBlockOpcode,
};

impl<'a> CompileNode<'a> for aria_parser::ast::MulOperation {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        self.left.do_compile(params)?;
        for right in &self.right {
            right.1.do_compile(params)?;
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(
                    match right.0 {
                        aria_parser::ast::MulSymbol::Star => BasicBlockOpcode::Mul,
                        aria_parser::ast::MulSymbol::Slash => BasicBlockOpcode::Div,
                        aria_parser::ast::MulSymbol::Percent => BasicBlockOpcode::Rem,
                    },
                    self.loc.clone(),
                );
        }
        Ok(())
    }
}
