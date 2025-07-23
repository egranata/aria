// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        Expression, ReturnStatement,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
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
