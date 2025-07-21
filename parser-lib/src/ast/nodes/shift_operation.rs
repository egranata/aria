// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        AddOperation, ShiftOperation, ShiftSymbol, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for ShiftOperation {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::shift);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        if inner.len() == 1 {
            let left = AddOperation::from_parse_tree(inner.next().expect("left operand"), source);
            Self {
                loc: source.pointer(loc),
                left,
                right: None,
            }
        } else if inner.len() == 3 {
            let left = AddOperation::from_parse_tree(inner.next().expect("left operand"), source);
            let op = ShiftSymbol::from_parse_tree(inner.next().expect("shift operator"), source);
            let right = AddOperation::from_parse_tree(inner.next().expect("right operand"), source);
            Self {
                loc: source.pointer(loc),
                left,
                right: Some((op, right)),
            }
        } else {
            panic!("invalid shift length");
        }
    }
}

impl PrettyPrintable for ShiftOperation {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        let this = self.left.prettyprint(buffer);
        if let Some(rhs) = &self.right {
            this << &rhs.0 << &rhs.1
        } else {
            this
        }
    }
}
