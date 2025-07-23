// SPDX-License-Identifier: Apache-2.0
use crate::do_compile::{CompilationResult, CompileNode, CompileParams, do_struct_compile};

impl<'a> CompileNode<'a> for aria_parser::ast::StructDecl {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        do_struct_compile(self, params)?;

        params.scope.emit_untyped_define(
            &self.name.value,
            &mut params.module.constants,
            params.writer.get_current_block(),
            self.loc.clone(),
        )?;

        Ok(())
    }
}
