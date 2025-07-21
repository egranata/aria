// SPDX-License-Identifier: Apache-2.0
use crate::{
    do_compile::{
        CompilationError, CompilationErrorReason, CompilationResult, CompileNode, CompileParams,
    },
    func_builder::BasicBlockOpcode,
};

impl<'a> CompileNode<'a> for aria_parser::ast::ContinueStatement {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        if let Some(continue_target) = &params.cflow.continue_dest {
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(
                    BasicBlockOpcode::Jump(continue_target.clone()),
                    self.loc.clone(),
                );
            Ok(())
        } else {
            Err(CompilationError {
                loc: self.loc.clone(),
                reason: CompilationErrorReason::FlowControlNotAllowed,
            })
        }
    }
}
