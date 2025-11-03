// SPDX-License-Identifier: Apache-2.0
use crate::do_compile::{CompilationResult, CompileNode, CompileParams};

impl<'a> CompileNode<'a> for aria_parser::ast::ValDeclStatement {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        for decl in &self.decls {
            decl.do_compile(params)?;
        }
        Ok(())
    }
}
