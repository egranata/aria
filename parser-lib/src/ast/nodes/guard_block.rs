// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        CodeBlock, Expression, GuardBlock, Identifier,
    },
    gen_from_components,
};

impl Derive for GuardBlock {
    gen_from_components!(guard_block; id: Identifier, expr: Expression, body: CodeBlock);
}

impl PrettyPrintable for GuardBlock {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "guard (" << &self.id << " = " << &self.expr << ") " << &self.body
    }
}
