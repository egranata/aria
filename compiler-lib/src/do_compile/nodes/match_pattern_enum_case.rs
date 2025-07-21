// SPDX-License-Identifier: Apache-2.0
use crate::{
    constant_value::ConstantValue,
    do_compile::{CompilationResult, CompileNode, CompileParams},
    func_builder::BasicBlockOpcode,
};

impl<'a> CompileNode<'a> for aria_parser::ast::MatchPatternEnumCase {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        let case_name_idx = self.insert_const_or_fail(
            params,
            ConstantValue::String(self.case.value.clone()),
            &self.loc,
        )?;
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(
                BasicBlockOpcode::EnumCheckIsCase(case_name_idx),
                self.loc.clone(),
            );
        if let Some(p) = &self.payload {
            let if_true = params.writer.insert_block_after(
                &format!("if_true_{}", self.loc),
                &params.writer.get_current_block(),
            );
            let if_false = params
                .writer
                .insert_block_after(&format!("if_false_{}", self.loc), &if_true);
            let if_payload_after = params
                .writer
                .insert_block_after(&format!("if_payload_after{}", self.loc), &if_false);
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(
                    BasicBlockOpcode::JumpTrue(if_true.clone()),
                    self.loc.clone(),
                );
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(
                    BasicBlockOpcode::Jump(if_false.clone()),
                    self.loc.clone(),
                );
            params.writer.set_current_block(if_false.clone());
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(BasicBlockOpcode::PushFalse, self.loc.clone());
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(
                    BasicBlockOpcode::Jump(if_payload_after.clone()),
                    self.loc.clone(),
                );
            params.writer.set_current_block(if_true);
            params.scope.emit_read(
                "__match_control_expr",
                &mut params.module.constants,
                params.writer.get_current_block(),
                p.loc.clone(),
            )?;
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(
                    BasicBlockOpcode::EnumExtractPayload,
                    self.loc.clone(),
                );
            if let Some(ty) = &p.ty {
                params
                    .writer
                    .get_current_block()
                    .write_opcode_and_source_info(BasicBlockOpcode::Dup, self.loc.clone());
                ty.do_compile(params)?;
                params
                    .writer
                    .get_current_block()
                    .write_opcode_and_source_info(BasicBlockOpcode::Isa, self.loc.clone())
                    .write_opcode_and_source_info(
                        BasicBlockOpcode::JumpFalse(if_false.clone()),
                        self.loc.clone(),
                    );
            }
            params.scope.emit_untyped_define(
                &p.name.value,
                &mut params.module.constants,
                params.writer.get_current_block(),
                self.loc.clone(),
            )?;
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(BasicBlockOpcode::PushTrue, self.loc.clone());
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(
                    BasicBlockOpcode::Jump(if_payload_after.clone()),
                    self.loc.clone(),
                );
            params.writer.set_current_block(if_payload_after);
        }
        Ok(())
    }
}
