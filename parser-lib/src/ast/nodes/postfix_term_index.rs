// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        ExpressionList, PostfixTermIndex,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_components,
};

impl Derive for PostfixTermIndex {
    gen_from_components!(postfix_term_index; index: ExpressionList);
}

impl PrettyPrintable for PostfixTermIndex {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "[" << &self.index << "]"
    }
}
