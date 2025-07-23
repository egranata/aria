// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        RelOperation, RelSymbol, ShiftOperation, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for RelOperation {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::rel);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        if inner.len() == 1 {
            let left = ShiftOperation::from_parse_tree(inner.next().expect("left operand"), source);
            Self {
                loc: source.pointer(loc),

                left,
                right: None,
            }
        } else if inner.len() == 3 {
            let left = ShiftOperation::from_parse_tree(inner.next().expect("left operand"), source);
            let op = RelSymbol::from_parse_tree(inner.next().expect("rel operator"), source);
            let right =
                ShiftOperation::from_parse_tree(inner.next().expect("right operand"), source);
            Self {
                loc: source.pointer(loc),
                left,
                right: Some((op, right)),
            }
        } else {
            panic!("invalid rel length");
        }
    }
}

impl PrettyPrintable for RelOperation {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        let this = self.left.prettyprint(buffer);
        if let Some(rhs) = &self.right {
            this << &rhs.0 << &rhs.1
        } else {
            this
        }
    }
}
