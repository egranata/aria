// SPDX-License-Identifier: Apache-2.0
use crate::{
    builder::compiler_opcodes::CompilerOpcode,
    do_compile::{CompilationResult, CompileNode, CompileParams},
};

impl<'a> CompileNode<'a> for aria_parser::ast::ThrowStatement {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        self.val.do_compile(params)?;
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(CompilerOpcode::Throw, self.loc.clone());
        Ok(())
    }
}
