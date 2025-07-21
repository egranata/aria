// SPDX-License-Identifier: Apache-2.0
use crate::do_compile::{CompilationResult, CompileNode, CompileParams};

impl<'a> CompileNode<'a> for aria_parser::ast::Identifier {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        params.scope.emit_read(
            &self.value,
            &mut params.module.constants,
            params.writer.get_current_block(),
            self.loc.clone(),
        )?;
        Ok(())
    }
}
