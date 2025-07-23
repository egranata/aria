// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        AssignStatement, Expression, PostfixExpression,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_components,
};

impl Derive for AssignStatement {
    gen_from_components!(val_write_stmt; id: PostfixExpression, val: Expression);
}

impl PrettyPrintable for AssignStatement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << &self.id << " = " << &self.val << ";"
    }
}
