// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        CodeBlock, IfCondExpr, IfCondPiece, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for IfCondPiece {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::if_cond_piece);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let expr = inner.next().expect("need expression");
        let body = inner.next().expect("need body");
        let expression = IfCondExpr::from_parse_tree(expr, source);
        let then = CodeBlock::from_parse_tree(body, source);
        Self {
            loc: source.pointer(loc),
            expression,
            then,
        }
    }
}

impl PrettyPrintable for IfCondPiece {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "(" << &self.expression << ")" << &self.then
    }
}
