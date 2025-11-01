// SPDX-License-Identifier: Apache-2.0
use crate::do_compile::{CompilationResult, CompileNode, CompileParams};

impl<'a> CompileNode<'a> for aria_parser::ast::FunctionBody {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        self.code.do_compile(params)
    }
}
