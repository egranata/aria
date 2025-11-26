// SPDX-License-Identifier: Apache-2.0
use crate::{
    builder::compiler_opcodes::CompilerOpcode,
    do_compile::{CompilationResult, CompileNode, CompileParams, do_enum_compile},
};

impl<'a> CompileNode<'a> for aria_parser::ast::EnumDecl {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        do_enum_compile(self, params, |name, params| {
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(CompilerOpcode::Dup, self.loc.clone());
            params
                .scope
                .emit_untyped_define(
                    name,
                    &mut params.module.constants,
                    params.writer.get_current_block(),
                    self.loc.clone(),
                )
                .map_err(Into::into)
        })
    }
}
