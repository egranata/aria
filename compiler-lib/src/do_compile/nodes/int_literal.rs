// SPDX-License-Identifier: Apache-2.0
use crate::{
    builder::compiler_opcodes::CompilerOpcode,
    constant_value::ConstantValue,
    do_compile::{
        CompilationError, CompilationErrorReason, CompilationResult, CompileNode, CompileParams,
    },
};

impl<'a> CompileNode<'a> for aria_parser::ast::IntLiteral {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        let inp_str = &self.val.replace('_', "");

        // I don't particularly like this code, but I don't know an easy way to write
        // if X { Result<A,E> } else if XX { Result<B,E> } ... . map_err(...)? as B;
        // basically, some paths want to generate a u64 and some want to generate an i64,
        // but all need to converge on i64 in the end.
        let val = if let Some(hex_str) = inp_str.strip_prefix("0x") {
            u64::from_str_radix(hex_str, 16).map_err(|_| CompilationError {
                loc: self.loc.clone(),
                reason: CompilationErrorReason::InvalidLiteral(inp_str.to_owned()),
            })? as i64
        } else if let Some(bin_str) = inp_str.strip_prefix("0b") {
            u64::from_str_radix(bin_str, 2).map_err(|_| CompilationError {
                loc: self.loc.clone(),
                reason: CompilationErrorReason::InvalidLiteral(inp_str.to_owned()),
            })? as i64
        } else if let Some(oct_str) = inp_str.strip_prefix("0o") {
            i64::from_str_radix(oct_str, 8).map_err(|_| CompilationError {
                loc: self.loc.clone(),
                reason: CompilationErrorReason::InvalidLiteral(inp_str.to_owned()),
            })?
        } else {
            inp_str.parse::<i64>().map_err(|_| CompilationError {
                loc: self.loc.clone(),
                reason: CompilationErrorReason::InvalidLiteral(inp_str.to_owned()),
            })?
        };

        if val == 0 {
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(CompilerOpcode::Push0, self.loc.clone());
        } else if val == 1 {
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(CompilerOpcode::Push1, self.loc.clone());
        } else {
            let const_idx =
                self.insert_const_or_fail(params, ConstantValue::Integer(val), &self.loc)?;
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(CompilerOpcode::Push(const_idx), self.loc.clone());
        }
        Ok(())
    }
}
