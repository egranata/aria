// SPDX-License-Identifier: Apache-2.0
use crate::do_compile::{CompilationResult, CompileNode, CompileParams};

impl<'a> CompileNode<'a> for aria_parser::ast::MatchPattern {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        match self {
            Self::MatchPatternRelational(e) => e.do_compile(params),
            Self::MatchPatternEnumCase(e) => e.do_compile(params),
        }
    }
}
