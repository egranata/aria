// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        CodeBlock, Expression, ForStatement, Identifier,
    },
    gen_from_components,
};

impl Derive for ForStatement {
    gen_from_components!(for_stmt; id: Identifier, expr: Expression, then: CodeBlock);
}

impl PrettyPrintable for ForStatement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "for( " << &self.id << " in " << &self.expr << ") " << &self.then
    }
}
