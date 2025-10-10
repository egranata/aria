// SPDX-License-Identifier: Apache-2.0

use crate::{
    do_compile::{CompilationResult, CompileNode, CompileParams, emit_type_members_compile},
    func_builder::BasicBlockOpcode,
};

impl<'a> CompileNode<'a> for aria_parser::ast::MixinDecl {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(BasicBlockOpcode::BuildMixin, self.loc.clone())
            .write_opcode_and_source_info(BasicBlockOpcode::Dup, self.loc.clone());
        params.scope.emit_untyped_define(
            &self.name.value,
            &mut params.module.constants,
            params.writer.get_current_block(),
            self.loc.clone(),
        )?;

        emit_type_members_compile(&self.body, params, true)
    }
}
