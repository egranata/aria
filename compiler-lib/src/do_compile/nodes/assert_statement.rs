// SPDX-License-Identifier: Apache-2.0
use aria_parser::ast::prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator};

use crate::{
    constant_value::ConstantValue,
    do_compile::{CompilationResult, CompileNode, CompileParams},
    func_builder::BasicBlockOpcode,
};

impl<'a> CompileNode<'a> for aria_parser::ast::AssertStatement {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        self.val.do_compile(params)?;
        let poa = PrintoutAccumulator::default();
        let poa = self.val.prettyprint(poa).value();
        let msg_idx = self.insert_const_or_fail(params, ConstantValue::String(poa), &self.loc)?;
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(BasicBlockOpcode::Assert(msg_idx), self.loc.clone());
        Ok(())
    }
}
