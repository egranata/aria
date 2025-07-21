// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        Expression, ExpressionStatement, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for ExpressionStatement {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::expr_stmt);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let val = Expression::from_parse_tree(inner.next().expect("need expression"), source);
        Self {
            loc: source.pointer(loc),
            val,
        }
    }
}

impl PrettyPrintable for ExpressionStatement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << &self.val << ";"
    }
}
