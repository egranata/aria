// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        AddEqSymbol, Expression, PostfixExpression, WriteOpEqStatement,
    },
    gen_from_components,
};

impl Derive for WriteOpEqStatement {
    gen_from_components!(val_add_eq_write; id: PostfixExpression, op: AddEqSymbol, val: Expression);
}

impl PrettyPrintable for WriteOpEqStatement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << &self.id << &self.op << &self.val << ";"
    }
}
