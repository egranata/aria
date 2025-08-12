// SPDX-License-Identifier: Apache-2.0

use haxby_opcodes::builtin_type_ids::BUILTIN_TYPE_UNIT;

use crate::{
    constant_value::{CompiledCodeObject, ConstantValue},
    do_compile::{
        CompilationError, CompilationErrorReason, CompilationResult, CompileNode, CompileParams,
        ControlFlowTargets, emit_args_at_target, ensure_unique_arg_names,
    },
    func_builder::{BasicBlockOpcode, FunctionBuilder},
    scope::CompilationScope,
};

impl<'a> CompileNode<'a> for aria_parser::ast::MethodDecl {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        if self.args.names.len() > u8::MAX.into() {
            return Err(CompilationError {
                loc: self.loc.clone(),
                reason: CompilationErrorReason::TooManyArguments,
            });
        }

        let f_scope = CompilationScope::function(params.scope);
        let cflow = ControlFlowTargets::default();
        let mut writer = FunctionBuilder::default();
        let mut c_params = CompileParams {
            module: params.module,
            scope: &f_scope,
            writer: &mut writer,
            cflow: &cflow,
            options: params.options,
        };

        ensure_unique_arg_names(&self.args)?;

        c_params.scope.emit_untyped_define(
            match self.access {
                aria_parser::ast::MethodAccess::Instance => "this",
                aria_parser::ast::MethodAccess::Type => "This",
            },
            &mut c_params.module.constants,
            c_params.writer.get_current_block(),
            self.loc.clone(),
        )?;
        emit_args_at_target(&self.args, &mut c_params)?;

        let unit = self.insert_const_or_fail(
            &mut c_params,
            ConstantValue::String("unit".to_owned()),
            &self.loc,
        )?;

        self.body.do_compile(&mut c_params)?;
        c_params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(
                BasicBlockOpcode::PushBuiltinTy(BUILTIN_TYPE_UNIT),
                self.loc.clone(),
            )
            .write_opcode_and_source_info(BasicBlockOpcode::NewEnumVal(unit), self.loc.clone())
            .write_opcode_and_source_info(BasicBlockOpcode::Return, self.loc.clone());

        let frame_size = c_params.scope.as_function_root().unwrap().num_locals();

        let co = match writer.write(&params.module.constants, params.options) {
            Ok(c) => c,
            Err(er) => {
                return Err(CompilationError {
                    loc: self.loc.clone(),
                    reason: er,
                });
            }
        };
        let line_table = writer.write_line_table().clone();
        let cco = CompiledCodeObject {
            name: self.name.value.clone(),
            body: co,
            arity: 1 + self.args.names.len() as u8,
            loc: self.loc.clone(),
            line_table,
            frame_size,
        };
        let cco_idx =
            self.insert_const_or_fail(params, ConstantValue::CompiledCodeObject(cco), &self.loc)?;
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(BasicBlockOpcode::Push(cco_idx), self.loc.clone());

        Ok(())
    }
}
