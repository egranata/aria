// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        Expression, LogOperation, SourceBuffer, TernaryExpression,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for TernaryExpression {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::ternary_expr);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let condition = LogOperation::from_parse_tree(inner.next().unwrap(), source);
        let true_expression = Expression::from_parse_tree(inner.next().unwrap(), source);
        let false_expression = Expression::from_parse_tree(inner.next().unwrap(), source);
        TernaryExpression {
            loc: source.pointer(loc),
            condition: Box::new(condition),
            true_expression: Box::new(true_expression),
            false_expression: Box::new(false_expression),
        }
    }
}

impl PrettyPrintable for TernaryExpression {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        self.condition.prettyprint(buffer)
            << " ? "
            << self.true_expression.as_ref()
            << " : "
            << self.false_expression.as_ref()
    }
}
