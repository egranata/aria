// SPDX-License-Identifier: Apache-2.0

use aria_parser::ast::SourcePointer;

use crate::{
    builder::compiler_opcodes::CompilerOpcode,
    constant_value::ConstantValue,
    do_compile::{CompilationResult, CompileNode, CompileParams},
};

fn emit_case_without_payload(
    _: &SourcePointer,
    case: &aria_parser::ast::Identifier,
    params: &mut CompileParams,
) -> CompilationResult {
    let case_name_idx =
        case.insert_const_or_fail(params, ConstantValue::String(case.value.clone()), &case.loc)?;
    params
        .writer
        .get_current_block()
        .write_opcode_and_source_info(
            CompilerOpcode::EnumCheckIsCase(case_name_idx),
            case.loc.clone(),
        );
    Ok(())
}

fn emit_case_with_payload(
    loc: &SourcePointer,
    case: &aria_parser::ast::Identifier,
    payload: &aria_parser::ast::DeclarationId,
    params: &mut CompileParams,
) -> CompilationResult {
    emit_case_without_payload(loc, case, params)?;
    // jump here when any intermediate check fails, this will push false on the stack
    let payload_check_failed = params
        .writer
        .append_block_at_end(&format!("payload_chck_failed{}", case.loc));
    // this is where match expects to continue, with either true or false on the stack
    // and possibly a local symbol bound on success
    let payload_check_aftermath = params
        .writer
        .append_block_at_end(&format!("payload_chck_aftermath{}", case.loc));
    params
        .writer
        .get_current_block()
        .write_opcode_and_source_info(
            CompilerOpcode::JumpFalse(payload_check_failed.clone()),
            loc.clone(),
        );
    // we know we have a case match - now extract the payload
    params.scope.emit_read(
        "__match_control_expr",
        &mut params.module.constants,
        params.writer.get_current_block(),
        payload.loc.clone(),
    )?;
    params
        .writer
        .get_current_block()
        .write_opcode_and_source_info(CompilerOpcode::EnumTryExtractPayload, payload.loc.clone());
    params
        .writer
        .get_current_block()
        .write_opcode_and_source_info(
            CompilerOpcode::JumpFalse(payload_check_failed.clone()),
            loc.clone(),
        );
    // if we're here, we know we have a payload - bind it to a local variable now
    if let Some(ty) = &payload.ty {
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(CompilerOpcode::Dup, loc.clone());
        ty.do_compile(params)?;
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(CompilerOpcode::Isa, loc.clone())
            .write_opcode_and_source_info(
                CompilerOpcode::JumpFalse(payload_check_failed.clone()),
                loc.clone(),
            );
    }
    params.scope.emit_untyped_define(
        &payload.name.value,
        &mut params.module.constants,
        params.writer.get_current_block(),
        loc.clone(),
    )?;
    // if we're still here, we passed all checks - push true and jump to aftermath
    params
        .writer
        .get_current_block()
        .write_opcode_and_source_info(CompilerOpcode::PushTrue, loc.clone())
        .write_opcode_and_source_info(
            CompilerOpcode::Jump(payload_check_aftermath.clone()),
            loc.clone(),
        );
    params.writer.set_current_block(payload_check_failed);
    params
        .writer
        .get_current_block()
        .write_opcode_and_source_info(CompilerOpcode::PushFalse, loc.clone())
        .write_opcode_and_source_info(
            CompilerOpcode::Jump(payload_check_aftermath.clone()),
            loc.clone(),
        );
    params.writer.set_current_block(payload_check_aftermath);
    Ok(())
}

impl<'a> CompileNode<'a> for aria_parser::ast::MatchPatternEnumCase {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        match &self.payload {
            None => emit_case_without_payload(&self.loc, &self.case, params),
            Some(decl_id) => emit_case_with_payload(&self.loc, &self.case, decl_id, params),
        }
    }
}
