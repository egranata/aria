// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        CodeBlock, Expression, GuardBlock, Identifier,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
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
