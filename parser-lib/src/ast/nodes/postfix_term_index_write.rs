// SPDX-License-Identifier: Apache-2.0
use crate::ast::{
    Expression, ExpressionList, PostfixTermIndexWrite,
    derive::Derive,
    prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
};

impl Derive for PostfixTermIndexWrite {
    fn from_parse_tree(
        p: pest::iterators::Pair<'_, crate::grammar::Rule>,
        source: &crate::ast::SourceBuffer,
    ) -> Self {
        assert!(p.as_rule() == crate::grammar::Rule::postfix_term_index_write);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let idx = if inner.peek().expect("need index").as_rule() == crate::grammar::Rule::expr_list
        {
            <ExpressionList>::from_parse_tree(
                inner.next().expect(concat!("need ", stringify!(idx))),
                source,
            )
        } else {
            ExpressionList::empty(source.pointer(loc))
        };
        let val = <Expression>::from_parse_tree(
            inner.next().expect(concat!("need ", stringify!(val))),
            source,
        );
        Self {
            loc: source.pointer(loc),
            idx,
            val,
        }
    }
}

impl PrettyPrintable for PostfixTermIndexWrite {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "[" << &self.idx << "] = " << &self.val
    }
}
