// SPDX-License-Identifier: Apache-2.0
use crate::{
    constant_value::ConstantValue,
    do_compile::{
        CompilationError, CompilationErrorReason, CompilationResult, CompileNode, CompileParams,
    },
    func_builder::BasicBlockOpcode,
};

impl<'a> CompileNode<'a> for aria_parser::ast::FloatLiteral {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        let fp_val: f64 = if let Some(fp_val) = self.val.strip_suffix("f") {
            match fp_val.parse::<f64>() {
                Ok(f) => f,
                Err(_) => {
                    return Err(CompilationError {
                        loc: self.loc.clone(),
                        reason: CompilationErrorReason::InvalidLiteral(self.val.clone()),
                    });
                }
            }
        } else {
            return Err(CompilationError {
                loc: self.loc.clone(),
                reason: CompilationErrorReason::InvalidLiteral(self.val.clone()),
            });
        };
        let const_idx =
            self.insert_const_or_fail(params, ConstantValue::Float(fp_val.into()), &self.loc)?;
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(BasicBlockOpcode::Push(const_idx), self.loc.clone());
        Ok(())
    }
}
