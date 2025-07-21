// SPDX-License-Identifier: Apache-2.0
use crate::do_compile::{CompilationResult, CompileNode, CompileParams};

impl<'a> CompileNode<'a> for aria_parser::ast::CodeBlock {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        let c_scope = params.scope.child();
        let mut c_params = CompileParams {
            module: params.module,
            scope: &c_scope,
            writer: params.writer,
            cflow: params.cflow,
            options: params.options,
        };

        for entry in &self.entries {
            entry.do_compile(&mut c_params)?;
        }
        Ok(())
    }
}
