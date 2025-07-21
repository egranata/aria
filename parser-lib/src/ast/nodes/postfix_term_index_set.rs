// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        Expression, PostfixTermIndexSet,
    },
    gen_from_components,
};

impl Derive for PostfixTermIndexSet {
    gen_from_components!(postfix_index_set; idx: Expression, val: Expression);
}

impl PrettyPrintable for PostfixTermIndexSet {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "[" << &self.idx << "] = " << &self.val
    }
}
