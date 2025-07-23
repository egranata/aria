// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        PostfixExpression, PostfixRvalue,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
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
