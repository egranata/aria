// SPDX-License-Identifier: Apache-2.0
use haxby_opcodes::enum_case_attribs::CASE_HAS_PAYLOAD;

use crate::{
    builder::compiler_opcodes::CompilerOpcode,
    constant_value::ConstantValue,
    do_compile::{CompilationResult, CompileNode, CompileParams},
};

impl<'a> CompileNode<'a> for aria_parser::ast::EnumCaseDecl {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult<()> {
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(CompilerOpcode::Dup, self.loc.clone());
        let attrib_byte = match &self.payload {
            Some(expr) => {
                expr.do_compile(params)?;
                CASE_HAS_PAYLOAD
            }
            None => 0,
        };
        let name_idx = self.insert_const_or_fail(
            params,
            ConstantValue::String(self.name.value.clone()),
            &self.loc,
        )?;
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(
                CompilerOpcode::BindCase(attrib_byte, name_idx),
                self.loc.clone(),
            );

        Ok(())
    }
}
