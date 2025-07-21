// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        PostfixTermContainerWrite, PostfixTermFieldIndexList,
    },
    gen_from_components,
};

impl Derive for PostfixTermContainerWrite {
    gen_from_components!(postfix_term_idx_write; terms: PostfixTermFieldIndexList);
}

impl PrettyPrintable for PostfixTermContainerWrite {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "{" << &self.terms << "}"
    }
}
