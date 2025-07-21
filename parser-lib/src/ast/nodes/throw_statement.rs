// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        Expression, ThrowStatement,
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
