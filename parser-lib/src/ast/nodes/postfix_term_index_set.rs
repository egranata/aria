// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        Expression, PostfixTermIndexSet,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
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
