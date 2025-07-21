// SPDX-License-Identifier: Apache-2.0
use crate::{
    do_compile::{CompilationResult, CompileNode, CompileParams},
    func_builder::BasicBlockOpcode,
};

impl<'a> CompileNode<'a> for aria_parser::ast::GuardBlock {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        let c_scope = params.scope.child();
        let mut c_params = CompileParams {
            module: params.module,
            scope: &c_scope,
            writer: params.writer,
            cflow: params.cflow,
            options: params.options,
        };

        self.expr.do_compile(&mut c_params)?;
        c_params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(BasicBlockOpcode::Dup, self.loc.clone());
        c_params.scope.emit_untyped_define(
            &self.id.value,
            &mut c_params.module.constants,
            c_params.writer.get_current_block(),
            self.id.loc.clone(),
        )?;

        c_params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(BasicBlockOpcode::GuardEnter, self.loc.clone());
        self.body.do_compile(&mut c_params)?;
        c_params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(BasicBlockOpcode::GuardExit, self.loc.clone());

        Ok(())
    }
}
