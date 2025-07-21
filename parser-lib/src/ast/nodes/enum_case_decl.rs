// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        EnumCaseDecl, Expression, Identifier, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for EnumCaseDecl {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::enum_case_decl);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let name = Identifier::from_parse_tree(inner.next().expect("need identifier"), source);
        let payload = inner
            .next()
            .map(|next| Expression::from_parse_tree(next, source));
        Self {
            loc: source.pointer(loc),
            name,
            payload,
        }
    }
}

impl PrettyPrintable for EnumCaseDecl {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        let buffer = buffer << &self.name;
        if let Some(payload) = &self.payload {
            buffer << "(" << payload << ")"
        } else {
            buffer
        }
    }
}
