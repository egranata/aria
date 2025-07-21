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
        let val = inner.next().map(|p| Expression::from_parse_tree(p, source));
        Self {
            loc: source.pointer(loc),
            val,
        }
    }
}

impl PrettyPrintable for ExpressionStatement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        if let Some(val) = &self.val {
            buffer << val << ";"
        } else {
            buffer << ";"
        }
    }
}
