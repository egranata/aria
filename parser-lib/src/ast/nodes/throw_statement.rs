// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        Expression, ThrowStatement,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_components,
};

impl Derive for ThrowStatement {
    gen_from_components!(throw_stmt; val: Expression);
}

impl PrettyPrintable for ThrowStatement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "throw " << &self.val << ";"
    }
}
