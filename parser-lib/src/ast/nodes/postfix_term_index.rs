// SPDX-License-Identifier: Apache-2.0
use crate::ast::{
    ExpressionList, PostfixTermIndex,
    derive::Derive,
    prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
};

impl Derive for PostfixTermIndex {
    fn from_parse_tree(
        p: pest::iterators::Pair<'_, crate::grammar::Rule>,
        source: &crate::ast::SourceBuffer,
    ) -> Self {
        assert!(p.as_rule() == crate::grammar::Rule::postfix_term_index);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let index = inner
            .next()
            .map_or(ExpressionList::empty(source.pointer(loc)), |e| {
                <ExpressionList>::from_parse_tree(e, source)
            });
        Self {
            loc: source.pointer(loc),
            index,
        }
    }
}

impl PrettyPrintable for PostfixTermIndex {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "[" << &self.index << "]"
    }
}
