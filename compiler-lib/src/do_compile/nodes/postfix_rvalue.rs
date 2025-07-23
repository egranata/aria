// SPDX-License-Identifier: Apache-2.0
use crate::do_compile::{CompilationResult, CompileNode, CompileParams, postfix::PostfixValue};

impl<'a> CompileNode<'a> for aria_parser::ast::PostfixRvalue {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        let pv = PostfixValue::from(&self.expr);
        pv.emit_read(params)
    }
}
