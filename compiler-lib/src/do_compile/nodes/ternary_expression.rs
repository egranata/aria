// SPDX-License-Identifier: Apache-2.0
use crate::{
    do_compile::{CompilationResult, CompileNode, CompileParams},
    func_builder::BasicBlockOpcode,
};
use aria_parser::ast::TernaryExpression;

impl<'a> CompileNode<'a> for TernaryExpression {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        self.condition.do_compile(params)?;

        let false_branch = params
            .writer
            .append_block_at_end(&format!("ternary_false_{}", self.loc));
        let end_branch = params
            .writer
            .append_block_at_end(&format!("ternary_end_{}", self.loc));

        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(
                BasicBlockOpcode::JumpFalse(false_branch.clone()),
                self.loc.clone(),
            );

        self.true_expression.do_compile(params)?;

        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(
                BasicBlockOpcode::Jump(end_branch.clone()),
                self.loc.clone(),
            );

        params.writer.set_current_block(false_branch);

        self.false_expression.do_compile(params)?;
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(
                BasicBlockOpcode::Jump(end_branch.clone()),
                self.loc.clone(),
            );

        params.writer.set_current_block(end_branch);

        Ok(())
    }
}
