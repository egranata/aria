// SPDX-License-Identifier: Apache-2.0
use crate::{
    do_compile::{CompilationResult, CompileNode, CompileParams},
    func_builder::BasicBlockOpcode,
};

impl<'a> CompileNode<'a> for aria_parser::ast::ThrowStatement {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        self.val.do_compile(params)?;
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(BasicBlockOpcode::Throw, self.loc.clone());
        Ok(())
    }
}
