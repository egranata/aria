// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        Expression, ParenExpression, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for ParenExpression {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::paren_expr);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let value = Expression::from_parse_tree(inner.next().expect("need expression"), source);
        Self {
            loc: source.pointer(loc),
            value: Box::new(value),
        }
    }
}

impl PrettyPrintable for ParenExpression {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "(" << &self.value << ")"
    }
}
