// SPDX-License-Identifier: Apache-2.0
use haxby_opcodes::builtin_type_ids::BUILTIN_TYPE_UNIT;

use crate::{
    constant_value::ConstantValue,
    do_compile::{CompilationResult, CompileNode, CompileParams},
    func_builder::BasicBlockOpcode,
};

impl<'a> CompileNode<'a> for aria_parser::ast::ReturnStatement {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        if let Some(val) = &self.val {
            val.do_compile(params)?;
        } else {
            let unit_name =
                self.insert_const_or_fail(params, ConstantValue::String("unit".into()), &self.loc)?;

            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(
                    BasicBlockOpcode::PushBuiltinTy(BUILTIN_TYPE_UNIT),
                    self.loc.clone(),
                )
                .write_opcode_and_source_info(
                    BasicBlockOpcode::NewEnumVal(unit_name),
                    self.loc.clone(),
                );
        }
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(BasicBlockOpcode::Return, self.loc.clone());
        Ok(())
    }
}
