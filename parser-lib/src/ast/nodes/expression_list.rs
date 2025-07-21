// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        Expression, ExpressionList, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for ExpressionList {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::expr_list);
        let loc = From::from(&p.as_span());
        let inner = p.into_inner();
        let expressions = inner
            .map(|e| Expression::from_parse_tree(e, source))
            .collect::<Vec<_>>();
        Self {
            loc: source.pointer(loc),
            expressions,
        }
    }
}

impl PrettyPrintable for ExpressionList {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write_separated_list(&self.expressions, ",")
    }
}
