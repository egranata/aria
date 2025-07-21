// SPDX-License-Identifier: Apache-2.0
use crate::do_compile::{emit_type_members_compile, CompilationResult, CompileNode, CompileParams};

impl<'a> CompileNode<'a> for aria_parser::ast::ExtensionDecl {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        self.target.do_compile(params)?;

        emit_type_members_compile(&self.body, params, true)
    }
}
