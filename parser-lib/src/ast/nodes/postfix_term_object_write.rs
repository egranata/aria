// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        PostfixTermFieldSetList, PostfixTermObjectWrite,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
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
