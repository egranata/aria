// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        PostfixExpression, PostfixRvalue,
    },
    gen_from_components,
};

impl Derive for PostfixRvalue {
    gen_from_components!(postfix_rv; expr: PostfixExpression);
}

impl PrettyPrintable for PostfixRvalue {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        self.expr.prettyprint(buffer)
    }
}
