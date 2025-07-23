// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        Identifier, PostfixTermAttribute,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_components,
};

impl Derive for PostfixTermAttribute {
    gen_from_components!(postfix_term_attrib; id: Identifier);
}

impl PrettyPrintable for PostfixTermAttribute {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "." << &self.id
    }
}
