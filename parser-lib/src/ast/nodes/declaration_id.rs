// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        DeclarationId, Expression, Identifier, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for DeclarationId {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::decl_id);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let name = Identifier::from_parse_tree(inner.next().expect("need identifier"), source);
        let ty = inner
            .next()
            .map(|next| Expression::from_parse_tree(next, source));
        Self {
            loc: source.pointer(loc),
            name,
            ty,
        }
    }
}

impl PrettyPrintable for DeclarationId {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        let buffer = buffer << &self.name;
        if let Some(ty) = &self.ty {
            buffer << " : " << ty
        } else {
            buffer
        }
    }
}
