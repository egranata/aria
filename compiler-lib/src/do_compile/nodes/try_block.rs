// SPDX-License-Identifier: Apache-2.0
use crate::{
    do_compile::{CompilationResult, CompileNode, CompileParams},
    func_builder::BasicBlockOpcode,
};

impl<'a> CompileNode<'a> for aria_parser::ast::TryBlock {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        let try_block = params.writer.insert_block_after(
            &format!("try_{}", &self.body.loc),
            &params.writer.get_current_block(),
        );
        let catch_block = params
            .writer
            .insert_block_after(&format!("catch_{}", &self.catch.loc), &try_block);
        let after_block = params
            .writer
            .insert_block_after(&format!("try_after_catch_{}", &self.body.loc), &catch_block);

        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(
                BasicBlockOpcode::Jump(try_block.clone()),
                self.loc.clone(),
            );
        params.writer.set_current_block(try_block);
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(
                BasicBlockOpcode::TryEnter(catch_block.clone()),
                self.loc.clone(),
            );
        self.body.do_compile(params)?;
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(BasicBlockOpcode::TryExit, self.loc.clone());
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(
                BasicBlockOpcode::Jump(after_block.clone()),
                self.loc.clone(),
            );
        params.writer.set_current_block(catch_block);

        let catch_scope = params.scope.child();
        let mut catch_params = CompileParams {
            module: params.module,
            scope: &catch_scope,
            writer: params.writer,
            cflow: params.cflow,
            options: params.options,
        };
        catch_params.scope.emit_untyped_define(
            &self.id.value,
            &mut catch_params.module.constants,
            catch_params.writer.get_current_block(),
            self.id.loc.clone(),
        )?;

        self.catch.do_compile(&mut catch_params)?;
        catch_params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(
                BasicBlockOpcode::Jump(after_block.clone()),
                self.loc.clone(),
            );
        catch_params.writer.set_current_block(after_block);

        Ok(())
    }
}
