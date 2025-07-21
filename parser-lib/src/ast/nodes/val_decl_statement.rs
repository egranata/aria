// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        DeclarationId, Expression, SourceBuffer, ValDeclStatement,
    },
    grammar::Rule,
};

impl Derive for ValDeclStatement {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::val_decl_stmt);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let id = DeclarationId::from_parse_tree(inner.next().expect("need identifier"), source);
        let val = Expression::from_parse_tree(inner.next().expect("need expression"), source);
        Self {
            loc: source.pointer(loc),
            id,
            val,
        }
    }
}

impl PrettyPrintable for ValDeclStatement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "val " << &self.id << " = " << &self.val << ";"
    }
}
