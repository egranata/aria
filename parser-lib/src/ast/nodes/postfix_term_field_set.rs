// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        Expression, Identifier, PostfixTermFieldSet,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
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
