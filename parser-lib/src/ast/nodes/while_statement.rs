// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        CodeBlock, ElsePiece, Expression, WhileStatement,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for WhileStatement {
    fn from_parse_tree(
        p: pest::iterators::Pair<'_, crate::grammar::Rule>,
        source: &crate::ast::SourceBuffer,
    ) -> Self {
        assert!(p.as_rule() == Rule::while_stmt);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let cond = Expression::from_parse_tree(inner.next().expect("need condition"), source);
        let then = CodeBlock::from_parse_tree(inner.next().expect("need then block"), source);
        let els = inner
            .next()
            .map(|else_p| ElsePiece::from_parse_tree(else_p, source));

        Self {
            loc: source.pointer(loc),
            cond,
            then,
            els,
        }
    }
}

impl PrettyPrintable for WhileStatement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "while " << &self.cond << &self.then
    }
}
