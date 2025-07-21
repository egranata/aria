// SPDX-License-Identifier: Apache-2.0
use crate::{
    do_compile::{CompilationResult, CompileNode, CompileParams},
    func_builder::BasicBlockOpcode,
};

impl<'a> CompileNode<'a> for aria_parser::ast::RelOperation {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        self.left.do_compile(params)?;
        if let Some(rhs) = &self.right {
            rhs.1.do_compile(params)?;
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(
                    match rhs.0 {
                        aria_parser::ast::RelSymbol::Greater => BasicBlockOpcode::GreaterThan,
                        aria_parser::ast::RelSymbol::Less => BasicBlockOpcode::LessThan,
                        aria_parser::ast::RelSymbol::GreaterEqual => {
                            BasicBlockOpcode::GreaterThanEqual
                        }
                        aria_parser::ast::RelSymbol::LessEqual => BasicBlockOpcode::LessThanEqual,
                    },
                    self.loc.clone(),
                );
        }
        Ok(())
    }
}
