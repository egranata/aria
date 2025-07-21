// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        ElsifPiece, IfCondPiece, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for ElsifPiece {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::elsif_piece);
        let mut inner = p.into_inner();
        let content = inner.next().expect("need content");
        let content = IfCondPiece::from_parse_tree(content, source);
        Self { content }
    }
}

impl PrettyPrintable for ElsifPiece {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "elsif " << &self.content
    }
}
