// SPDX-License-Identifier: Apache-2.0
use crate::{
    do_compile::{CompilationResult, CompileNode, CompileParams, ControlFlowTargets},
    func_builder::BasicBlockOpcode,
};

impl<'a> CompileNode<'a> for aria_parser::ast::WhileStatement {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        let check = params
            .writer
            .append_block_at_end(&format!("check_{}", self.loc));
        let then = params
            .writer
            .append_block_at_end(&format!("then_{}", self.loc));
        let after = params
            .writer
            .append_block_at_end(&format!("after_{}", self.loc));

        let w_cflow = ControlFlowTargets {
            break_dest: Some(after.clone()),
            continue_dest: Some(check.clone()),
        };

        let mut c_params = CompileParams {
            module: params.module,
            scope: params.scope,
            writer: params.writer,
            cflow: &w_cflow,
            options: params.options,
        };

        c_params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(BasicBlockOpcode::Jump(check.clone()), self.loc.clone());
        c_params.writer.set_current_block(check.clone());
        self.cond.do_compile(&mut c_params)?;
        c_params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(
                BasicBlockOpcode::JumpTrue(then.clone()),
                self.then.loc.clone(),
            );
        c_params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(BasicBlockOpcode::Jump(after.clone()), self.loc.clone());
        c_params.writer.set_current_block(then);
        self.then.do_compile(&mut c_params)?;
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(BasicBlockOpcode::Jump(check.clone()), self.loc.clone());
        params.writer.set_current_block(after);
        Ok(())
    }
}
