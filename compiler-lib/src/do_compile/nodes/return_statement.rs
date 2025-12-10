// SPDX-License-Identifier: Apache-2.0
use crate::{
    builder::compiler_opcodes::CompilerOpcode,
    do_compile::{CompilationResult, CompileNode, CompileParams},
};

impl<'a> CompileNode<'a> for aria_parser::ast::ReturnStatement {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        if let Some(val) = &self.val {
            val.do_compile(params)?;
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(CompilerOpcode::Return, self.loc.clone());
        } else {
            self.return_unit_value(params, &self.loc)?;
        }
        Ok(())
    }
}
