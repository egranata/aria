// SPDX-License-Identifier: Apache-2.0
use crate::{
    do_compile::{
        emit_args_at_target, ensure_unique_arg_names, CompilationResult, CompileNode, CompileParams,
    },
    func_builder::BasicBlockOpcode,
};

impl<'a> CompileNode<'a> for aria_parser::ast::MethodDecl {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        ensure_unique_arg_names(&self.args)?;

        params.scope.emit_untyped_define(
            match self.access {
                aria_parser::ast::MethodAccess::Instance => "this",
                aria_parser::ast::MethodAccess::Type => "This",
            },
            &mut params.module.constants,
            params.writer.get_current_block(),
            self.loc.clone(),
        )?;

        emit_args_at_target(&self.args, params)?;

        if self.vararg {
            params.scope.emit_untyped_define(
                "varargs",
                &mut params.module.constants,
                params.writer.get_current_block(),
                self.loc.clone(),
            )?;
        }

        self.body.do_compile(params)?;
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(BasicBlockOpcode::Return, self.loc.clone());
        Ok(())
    }
}
