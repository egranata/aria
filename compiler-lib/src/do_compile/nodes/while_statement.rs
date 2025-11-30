// SPDX-License-Identifier: Apache-2.0
use crate::{
    builder::compiler_opcodes::CompilerOpcode,
    do_compile::{CompilationResult, CompileNode, CompileParams, ControlFlowTargets},
};

impl<'a> CompileNode<'a> for aria_parser::ast::WhileStatement {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        let first_check = params
            .writer
            .append_block_at_end(&format!("first_check_{}", self.loc));
        let check = params
            .writer
            .append_block_at_end(&format!("check_{}", self.loc));
        let then = params
            .writer
            .append_block_at_end(&format!("then_{}", self.loc));
        let els = params
            .writer
            .append_block_at_end(&format!("else_{}", self.loc));
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

        // the logic here is a bit tricky because of the else:
        // jump to first_check, which will check the condition
        // if true, jump to then, which will execute the body
        // if false, jump to else, which will then jump to after
        // then will jump back to check, which will check the condition again
        // if the condition is still true, it will jump to then again
        // if the condition is false, it will jump to after (not to else)

        c_params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(
                CompilerOpcode::Jump(first_check.clone()),
                self.loc.clone(),
            );
        c_params.writer.set_current_block(first_check.clone());
        self.cond.do_compile(&mut c_params)?;
        c_params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(
                CompilerOpcode::JumpTrue(then.clone()),
                self.then.loc.clone(),
            );
        c_params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(CompilerOpcode::Jump(els.clone()), self.loc.clone());

        c_params.writer.set_current_block(check.clone());
        self.cond.do_compile(&mut c_params)?;
        c_params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(
                CompilerOpcode::JumpTrue(then.clone()),
                self.then.loc.clone(),
            );
        c_params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(CompilerOpcode::Jump(after.clone()), self.loc.clone());
        c_params.writer.set_current_block(then);
        self.then.do_compile(&mut c_params)?;
        c_params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(CompilerOpcode::Jump(check.clone()), self.loc.clone());

        c_params.writer.set_current_block(els);
        if let Some(els) = &self.els {
            els.then.do_compile(&mut c_params)?;
        }
        c_params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(CompilerOpcode::Jump(after.clone()), self.loc.clone());
        c_params.writer.set_current_block(after);
        Ok(())
    }
}
