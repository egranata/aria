// SPDX-License-Identifier: Apache-2.0
use crate::do_compile::{CompilationResult, CompileNode, CompileParams};

impl<'a> CompileNode<'a, usize> for aria_parser::ast::ExpressionList {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult<usize> {
        for expr in &self.expressions {
            expr.do_compile(params)?;
        }
        Ok(self.expressions.len())
    }
}
