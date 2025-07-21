// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        PostfixExpression, PostfixTerm, Primary, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for PostfixExpression {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::postfix_lv);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let base = Primary::from_parse_tree(inner.next().expect("postfix need base"), source);
        let mut terms = vec![];
        loop {
            let next = inner.next();
            if next.is_none() {
                break;
            };
            let next = PostfixTerm::from_parse_tree(next.expect("need postfix term"), source);
            terms.push(next);
        }
        Self {
            loc: source.pointer(loc),
            base,
            terms,
        }
    }
}

impl PrettyPrintable for PostfixExpression {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        let mut this = self.base.prettyprint(buffer);
        for term in &self.terms {
            this = term.prettyprint(this);
        }
        this
    }
}
