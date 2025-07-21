// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        AssertStatement, Expression,
    },
    gen_from_components,
};

impl Derive for AssertStatement {
    gen_from_components!(assert_stmt; val: Expression);
}

impl PrettyPrintable for AssertStatement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "assert " << &self.val << ";"
    }
}
