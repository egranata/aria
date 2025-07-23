// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        ExpressionList, ListLiteral, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for ListLiteral {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::list_literal);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let items = if let Some(next) = inner.next() {
            ExpressionList::from_parse_tree(next, source)
        } else {
            ExpressionList::empty(source.pointer(loc))
        };
        Self {
            loc: source.pointer(loc),
            items,
        }
    }
}

impl PrettyPrintable for ListLiteral {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "[" << &self.items << "]"
    }
}
