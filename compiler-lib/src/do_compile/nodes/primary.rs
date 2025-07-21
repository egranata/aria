// SPDX-License-Identifier: Apache-2.0
use crate::do_compile::{CompilationResult, CompileNode, CompileParams};

impl<'a> CompileNode<'a> for aria_parser::ast::Primary {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        match self {
            Self::IntLiteral(il) => il.do_compile(params),
            Self::FloatLiteral(fp) => fp.do_compile(params),
            Self::Identifier(id) => id.do_compile(params),
            Self::ListLiteral(ll) => ll.do_compile(params),
            Self::StringLiteral(sl) => sl.do_compile(params),
            Self::ParenExpression(pe) => pe.do_compile(params),
        }
    }
}
