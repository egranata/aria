// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        PostfixTermObjectWrite, PostfixTermWriteList,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_components,
};

impl Derive for PostfixTermObjectWrite {
    gen_from_components!(postfix_term_object_write; terms: PostfixTermWriteList);
}

impl PrettyPrintable for PostfixTermObjectWrite {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "{" << &self.terms << "}"
    }
}
