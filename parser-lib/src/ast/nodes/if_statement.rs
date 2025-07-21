// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        ElsePiece, ElsifPiece, IfPiece, IfStatement, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for IfStatement {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::if_stmt);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let ifpiece = inner.next().expect("need if piece");
        let iff = IfPiece::from_parse_tree(ifpiece, source);
        let mut elsif_pieces = vec![];
        loop {
            let p = inner.peek();
            if p.is_none() {
                break;
            }
            if p.unwrap().as_rule() != Rule::elsif_piece {
                break;
            }
            let p = inner.next().unwrap();
            let elsif_rule = ElsifPiece::from_parse_tree(p, source);
            elsif_pieces.push(elsif_rule);
        }
        let p = inner.peek();
        let els = if p.is_none() {
            None
        } else {
            let els_piece = inner.next().unwrap();
            let els = ElsePiece::from_parse_tree(els_piece, source);
            Some(els)
        };
        Self {
            loc: source.pointer(loc),
            iff,
            elsif: elsif_pieces,
            els,
        }
    }
}

impl PrettyPrintable for IfStatement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        let mut this = self.iff.prettyprint(buffer);
        for elsif in &self.elsif {
            this = elsif.prettyprint(this);
        }
        this << &self.els
    }
}
