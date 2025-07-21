// SPDX-License-Identifier: Apache-2.0
use crate::do_compile::{CompilationResult, CompileNode, CompileParams};

impl<'a> CompileNode<'a> for aria_parser::ast::ParenExpression {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        self.value.do_compile(params)
    }
}
