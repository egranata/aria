// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        ArgumentDecl, DeclarationId, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for ArgumentDecl {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::arg_decl);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let id = DeclarationId::from_parse_tree(inner.next().expect("expected decl_id"), source);
        Self {
            loc: source.pointer(loc),
            id,
        }
    }
}

impl PrettyPrintable for ArgumentDecl {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << &self.id
    }
}
