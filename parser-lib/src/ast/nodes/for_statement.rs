// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        CodeBlock, ElsePiece, Expression, ForStatement, Identifier,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for ForStatement {
    fn from_parse_tree(
        p: pest::iterators::Pair<'_, crate::grammar::Rule>,
        source: &crate::ast::SourceBuffer,
    ) -> Self {
        assert!(p.as_rule() == Rule::for_stmt);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let id = Identifier::from_parse_tree(inner.next().expect("need identifier"), source);
        let expr = Expression::from_parse_tree(inner.next().expect("need expression"), source);
        let then = CodeBlock::from_parse_tree(inner.next().expect("need then block"), source);
        let els = inner
            .next()
            .map(|else_p| ElsePiece::from_parse_tree(else_p, source));

        Self {
            loc: source.pointer(loc),
            id,
            expr,
            then,
            els,
        }
    }
}

impl PrettyPrintable for ForStatement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "for( " << &self.id << " in " << &self.expr << ") " << &self.then
    }
}
