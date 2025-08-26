// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        Expression, PostfixTermIndexWrite,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_components,
};

impl Derive for PostfixTermIndexWrite {
    gen_from_components!(postfix_term_index_write; idx: Expression, val: Expression);
}

impl PrettyPrintable for PostfixTermIndexWrite {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "[" << &self.idx << "] = " << &self.val
    }
}
