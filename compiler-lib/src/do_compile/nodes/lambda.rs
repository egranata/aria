// SPDX-License-Identifier: Apache-2.0
use aria_parser::ast::{FunctionBody, FunctionDecl, Identifier};

use crate::do_compile::{CompilationResult, CompileNode, CompileParams};

impl<'a> CompileNode<'a> for aria_parser::ast::LambdaFunction {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        let body: FunctionBody = From::from(self.body.as_ref());
        let f_name = format!("<anon_f_{}>", self.loc);
        let f_obj = FunctionDecl {
            loc: body.loc().clone(),
            name: Identifier {
                loc: self.loc.clone(),
                value: f_name.clone(),
            },
            args: self.args.clone(),
            body,
        };

        let f_body_scope = params.scope.closure(params.writer.get_current_block());
        let mut f_body_params = CompileParams {
            module: params.module,
            scope: &f_body_scope,
            writer: params.writer,
            cflow: params.cflow,
            options: params.options,
        };

        f_obj.do_compile(&mut f_body_params)?;

        params.scope.emit_read(
            &f_name,
            &mut params.module.constants,
            params.writer.get_current_block(),
            self.loc.clone(),
        )?;
        Ok(())
    }
}
