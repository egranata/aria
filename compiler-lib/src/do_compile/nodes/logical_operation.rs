// SPDX-License-Identifier: Apache-2.0
use crate::{
    builder::compiler_opcodes::CompilerOpcode,
    do_compile::{CompilationResult, CompileNode, CompileParams},
};

impl<'a> CompileNode<'a> for aria_parser::ast::LogOperation {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        self.left.do_compile(params)?;
        for right in &self.right {
            match right.0 {
                // xor and bitwise operators do not shortcircut
                aria_parser::ast::LogSymbol::Ampersand => {
                    right.1.do_compile(params)?;
                    params
                        .writer
                        .get_current_block()
                        .write_opcode_and_source_info(CompilerOpcode::BitwiseAnd, self.loc.clone());
                }
                aria_parser::ast::LogSymbol::Pipe => {
                    right.1.do_compile(params)?;
                    params
                        .writer
                        .get_current_block()
                        .write_opcode_and_source_info(CompilerOpcode::BitwiseOr, self.loc.clone());
                }
                aria_parser::ast::LogSymbol::Caret => {
                    right.1.do_compile(params)?;
                    params
                        .writer
                        .get_current_block()
                        .write_opcode_and_source_info(CompilerOpcode::Xor, self.loc.clone());
                }
                aria_parser::ast::LogSymbol::DoubleAmpersand => {
                    let bb_and_true = params
                        .writer
                        .append_block_at_end(&format!("bb_and_true{}", self.loc));
                    let bb_and_done = params
                        .writer
                        .append_block_at_end(&format!("bb_and_done{}", self.loc));
                    params
                        .writer
                        .get_current_block()
                        .write_opcode_and_source_info(
                            CompilerOpcode::JumpTrue(bb_and_true.clone()),
                            self.loc.clone(),
                        );
                    params
                        .writer
                        .get_current_block()
                        .write_opcode_and_source_info(CompilerOpcode::PushFalse, self.loc.clone());
                    params
                        .writer
                        .get_current_block()
                        .write_opcode_and_source_info(
                            CompilerOpcode::Jump(bb_and_done.clone()),
                            self.loc.clone(),
                        );
                    params.writer.set_current_block(bb_and_true);
                    right.1.do_compile(params)?; // true and X == X
                    params
                        .writer
                        .get_current_block()
                        .write_opcode_and_source_info(
                            CompilerOpcode::Jump(bb_and_done.clone()),
                            self.loc.clone(),
                        );
                    params.writer.set_current_block(bb_and_done);
                }
                aria_parser::ast::LogSymbol::DoublePipe => {
                    let bb_or_true = params
                        .writer
                        .append_block_at_end(&format!("bb_or_true{}", self.loc));
                    let bb_or_done = params
                        .writer
                        .append_block_at_end(&format!("bb_or_done{}", self.loc));
                    params
                        .writer
                        .get_current_block()
                        .write_opcode_and_source_info(
                            CompilerOpcode::JumpTrue(bb_or_true.clone()),
                            self.loc.clone(),
                        );
                    right.1.do_compile(params)?; // false or X == X
                    params
                        .writer
                        .get_current_block()
                        .write_opcode_and_source_info(
                            CompilerOpcode::Jump(bb_or_done.clone()),
                            self.loc.clone(),
                        );
                    params.writer.set_current_block(bb_or_true);
                    params
                        .writer
                        .get_current_block()
                        .write_opcode_and_source_info(CompilerOpcode::PushTrue, self.loc.clone());
                    params
                        .writer
                        .get_current_block()
                        .write_opcode_and_source_info(
                            CompilerOpcode::Jump(bb_or_done.clone()),
                            self.loc.clone(),
                        );
                    params.writer.set_current_block(bb_or_done);
                }
            }
        }
        Ok(())
    }
}
