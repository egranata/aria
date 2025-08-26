// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        PostfixTerm,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_options,
};

use crate::ast::{
    PostfixTermAttribute, PostfixTermCall, PostfixTermEnumCase, PostfixTermIndex,
    PostfixTermObjectWrite,
};

impl Derive for PostfixTerm {
    gen_from_options!(
        postfix_term;
        (postfix_term_attrib, PostfixTermAttribute),
        (postfix_term_index, PostfixTermIndex),
        (postfix_term_call, PostfixTermCall),
        (postfix_term_enum_case, PostfixTermEnumCase),
        (postfix_term_object_write, PostfixTermObjectWrite),
    );
}

impl PrettyPrintable for PostfixTerm {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        match self {
            Self::PostfixTermAttribute(a) => a.prettyprint(buffer),
            Self::PostfixTermIndex(i) => i.prettyprint(buffer),
            Self::PostfixTermCall(c) => c.prettyprint(buffer),
            Self::PostfixTermEnumCase(c) => c.prettyprint(buffer),
            Self::PostfixTermObjectWrite(w) => w.prettyprint(buffer),
        }
    }
}
