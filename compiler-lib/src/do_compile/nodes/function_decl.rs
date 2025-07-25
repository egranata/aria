// SPDX-License-Identifier: Apache-2.0
use haxby_opcodes::{builtin_type_ids::BUILTIN_TYPE_UNIT, function_attribs::FUNC_ACCEPTS_VARARG};

use crate::{
    constant_value::{CompiledCodeObject, ConstantValue},
    do_compile::{
        CompilationError, CompilationErrorReason, CompilationResult, CompileNode, CompileParams,
        ControlFlowTargets, emit_args_at_target, ensure_unique_arg_names,
    },
    func_builder::{BasicBlockOpcode, FunctionBuilder},
};

impl<'a> CompileNode<'a> for aria_parser::ast::FunctionDecl {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        if self.args.names.len() > u8::MAX.into() {
            return Err(CompilationError {
                loc: self.loc.clone(),
                reason: CompilationErrorReason::TooManyArguments,
            });
        }

        let cflow = ControlFlowTargets::default();
        let mut writer = FunctionBuilder::default();
        let mut c_params = CompileParams {
            module: params.module,
            scope: params.scope,
            writer: &mut writer,
            cflow: &cflow,
            options: params.options,
        };

        ensure_unique_arg_names(&self.args)?;

        emit_args_at_target(&self.args, &mut c_params)?;
        if self.vararg {
            c_params.scope.emit_untyped_define(
                "varargs",
                &mut c_params.module.constants,
                c_params.writer.get_current_block(),
                self.loc.clone(),
            )?;
        }

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

        let co = match writer.write(&params.module.constants, params.options) {
            Ok(c) => c,
            Err(er) => {
                return Err(CompilationError {
                    loc: self.loc.clone(),
                    reason: er,
                });
            }
        };
        let frame_size = params.scope.as_function_root().unwrap().num_locals();
        let line_table = writer.write_line_table().clone();
        let cco = CompiledCodeObject {
            name: self.name.value.clone(),
            body: co,
            arity: self.args.names.len() as u8,
            loc: self.loc.clone(),
            line_table,
            frame_size,
        };
        let cco_idx =
            self.insert_const_or_fail(params, ConstantValue::CompiledCodeObject(cco), &self.loc)?;
        let a = if self.vararg {
            FUNC_ACCEPTS_VARARG
        } else {
            0_u8
        };
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(BasicBlockOpcode::Push(cco_idx), self.loc.clone())
            .write_opcode_and_source_info(BasicBlockOpcode::BuildFunction(a), self.loc.clone());

        for uplv in params
            .scope
            .as_function_root()
            .unwrap()
            .uplevels
            .borrow()
            .iter()
        {
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(
                    BasicBlockOpcode::StoreUplevel(uplv.idx_in_uplevel),
                    self.loc.clone(),
                );
        }

        params
            .scope
            .get_module_scope()
            .unwrap()
            .emit_untyped_define(
                &self.name.value,
                &mut params.module.constants,
                params.writer.get_current_block(),
                self.loc.clone(),
            )?;
        Ok(())
    }
}
