// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        Expression, Identifier, PostfixTermFieldSet,
    },
    gen_from_components,
};

impl Derive for PostfixTermFieldSet {
    gen_from_components!(postfix_field_set; id: Identifier, val: Expression);
}

impl PrettyPrintable for PostfixTermFieldSet {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "." << &self.id << " = " << &self.val
    }
}
