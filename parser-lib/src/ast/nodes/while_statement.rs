// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        CodeBlock, Expression, WhileStatement,
    },
    gen_from_components,
};

impl Derive for WhileStatement {
    gen_from_components!(while_stmt; cond: Expression, then: CodeBlock);
}

impl PrettyPrintable for WhileStatement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "while " << &self.cond << &self.then
    }
}
