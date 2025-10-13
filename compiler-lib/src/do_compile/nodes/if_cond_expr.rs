// SPDX-License-Identifier: Apache-2.0
use crate::do_compile::{CompilationResult, CompileNode, CompileParams};

impl<'a> CompileNode<'a> for aria_parser::ast::IfCondExpr {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        match self {
            aria_parser::ast::IfCondExpr::IfCondCase(p) => p.do_compile(params),
            aria_parser::ast::IfCondExpr::Expression(e) => e.do_compile(params),
        }
    }
}
