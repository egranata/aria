// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        Expression, ReturnStatement,
    },
    gen_from_components,
};

impl Derive for ReturnStatement {
    gen_from_components!(return_stmt; val: Expression);
}

impl PrettyPrintable for ReturnStatement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "return " << &self.val << ";"
    }
}
