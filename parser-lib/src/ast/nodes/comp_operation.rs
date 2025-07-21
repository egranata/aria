// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        CompOperation, CompSymbol, RelOperation, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for CompOperation {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::comp);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        if inner.len() == 1 {
            let left = RelOperation::from_parse_tree(inner.peek().expect("need a add"), source);
            Self {
                loc: source.pointer(loc),
                left,
                right: None,
            }
        } else {
            let left = RelOperation::from_parse_tree(inner.next().expect("need first add"), source);
            let op = CompSymbol::from_parse_tree(inner.next().expect("need op"), source);
            let right =
                RelOperation::from_parse_tree(inner.next().expect("need second add"), source);
            Self {
                loc: source.pointer(loc),
                left,
                right: Some((op, right)),
            }
        }
    }
}

impl PrettyPrintable for CompOperation {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        let this = self.left.prettyprint(buffer);
        if let Some(right) = &self.right {
            this << &right.0 << &right.1
        } else {
            this
        }
    }
}
