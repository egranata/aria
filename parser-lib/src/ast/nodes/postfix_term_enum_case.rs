// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        Expression, Identifier, PostfixTermEnumCase, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for PostfixTermEnumCase {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::postfix_term_enum_case);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let id = Identifier::from_parse_tree(inner.next().expect("need identifier"), source);
        let payload = inner
            .next()
            .map(|next| Expression::from_parse_tree(next, source));
        Self {
            loc: source.pointer(loc),
            id,
            payload,
        }
    }
}

impl PrettyPrintable for PostfixTermEnumCase {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        let buffer = buffer << "::" << &self.id;
        if let Some(payload) = &self.payload {
            buffer << "(" << payload << ")"
        } else {
            buffer
        }
    }
}
