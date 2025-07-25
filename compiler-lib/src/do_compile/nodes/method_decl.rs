// SPDX-License-Identifier: Apache-2.0

use haxby_opcodes::builtin_type_ids::BUILTIN_TYPE_UNIT;

use crate::{
    constant_value::ConstantValue,
    do_compile::{
        CompilationResult, CompileNode, CompileParams, emit_args_at_target, ensure_unique_arg_names,
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

        let unit =
            self.insert_const_or_fail(params, ConstantValue::String("unit".to_owned()), &self.loc)?;

        self.body.do_compile(params)?;
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(
                BasicBlockOpcode::PushBuiltinTy(BUILTIN_TYPE_UNIT),
                self.loc.clone(),
            )
            .write_opcode_and_source_info(BasicBlockOpcode::NewEnumVal(unit), self.loc.clone())
            .write_opcode_and_source_info(BasicBlockOpcode::Return, self.loc.clone());
        Ok(())
    }
}
