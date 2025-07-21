// SPDX-License-Identifier: Apache-2.0
use crate::{
    do_compile::{CompilationResult, CompileNode, CompileParams},
    func_builder::BasicBlockOpcode,
};

impl<'a> CompileNode<'a> for aria_parser::ast::MatchStatement {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        let c_scope = params.scope.child();
        let mut match_param = CompileParams {
            module: params.module,
            scope: &c_scope,
            writer: params.writer,
            cflow: params.cflow,
            options: params.options,
        };

        self.expr.do_compile(&mut match_param)?;

        // store the control expression here so it can be used
        match_param.scope.emit_untyped_define(
            "__match_control_expr",
            &mut match_param.module.constants,
            match_param.writer.get_current_block(),
            self.loc.clone(),
        )?;

        let match_after = match_param
            .writer
            .append_block_at_end(&format!("match_after_{}", self.loc));

        for rule in &self.rules {
            let r_scope = match_param.scope.child();
            let mut rule_param = CompileParams {
                module: match_param.module,
                scope: &r_scope,
                writer: match_param.writer,
                cflow: match_param.cflow,
                options: match_param.options,
            };

            let match_hit = rule_param.writer.insert_block_after(
                &format!("match_hit_{}", rule.loc),
                &rule_param.writer.get_current_block(),
            );
            let match_miss = rule_param
                .writer
                .insert_block_after(&format!("match_miss_{}", rule.loc), &match_hit);

            for pattern in &rule.patterns {
                rule_param.scope.emit_read(
                    "__match_control_expr",
                    &mut rule_param.module.constants,
                    rule_param.writer.get_current_block(),
                    pattern.loc().clone(),
                )?;
                // pattern is going to leave true (hit) or false (miss)
                // and may add local variables to the scope
                pattern.do_compile(&mut rule_param)?;
                rule_param
                    .writer
                    .get_current_block()
                    .write_opcode_and_source_info(
                        BasicBlockOpcode::JumpFalse(match_miss.clone()),
                        pattern.loc().clone(),
                    );
            }
            rule_param
                .writer
                .get_current_block()
                .write_opcode_and_source_info(
                    BasicBlockOpcode::Jump(match_hit.clone()),
                    rule.loc.clone(),
                );
            rule_param.writer.set_current_block(match_hit);
            rule.then.do_compile(&mut rule_param)?;
            rule_param
                .writer
                .get_current_block()
                .write_opcode_and_source_info(
                    BasicBlockOpcode::Jump(match_after.clone()),
                    rule.loc.clone(),
                );
            rule_param.writer.set_current_block(match_miss);
        }

        if let Some(els) = &self.els {
            els.then.do_compile(&mut match_param)?;
        }
        match_param
            .writer
            .get_current_block()
            .write_opcode_and_source_info(
                BasicBlockOpcode::Jump(match_after.clone()),
                self.loc.clone(),
            );
        match_param.writer.set_current_block(match_after);

        Ok(())
    }
}
