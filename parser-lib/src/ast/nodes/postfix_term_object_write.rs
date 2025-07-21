// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        PostfixTermFieldSetList, PostfixTermObjectWrite,
    },
    gen_from_components,
};

impl Derive for PostfixTermObjectWrite {
    gen_from_components!(postfix_term_dot_write; terms: PostfixTermFieldSetList);
}

impl PrettyPrintable for PostfixTermObjectWrite {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "{" << &self.terms << "}"
    }
}
