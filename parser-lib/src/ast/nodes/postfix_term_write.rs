// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        PostfixTermFieldWrite, PostfixTermIndexWrite, PostfixTermWrite,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_options,
};

impl Derive for PostfixTermWrite {
    gen_from_options!(postfix_term_write; (postfix_term_field_write, PostfixTermFieldWrite), (postfix_term_index_write, PostfixTermIndexWrite));
}

impl PrettyPrintable for PostfixTermWrite {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        match self {
            Self::PostfixTermFieldWrite(v) => v.prettyprint(buffer),
            Self::PostfixTermIndexWrite(v) => v.prettyprint(buffer),
        }
    }
}
