// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        CodeBlock, ElsePiece, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for ElsePiece {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::else_piece);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let body = inner.next().expect("need body");
        let then = CodeBlock::from_parse_tree(body, source);
        Self {
            loc: source.pointer(loc),
            then,
        }
    }
}

impl PrettyPrintable for ElsePiece {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "else " << &self.then
    }
}
