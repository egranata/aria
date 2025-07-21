// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        Identifier, MixinDecl, MixinEntry, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for MixinDecl {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::mixin_decl);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let name = Identifier::from_parse_tree(inner.next().expect("need identifier"), source);
        let mut body = vec![];
        for next in inner {
            let next = MixinEntry::from_parse_tree(next, source);
            body.push(next);
        }
        Self {
            loc: source.pointer(loc),
            name,
            body,
        }
    }
}

impl PrettyPrintable for MixinDecl {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        (buffer << "mixin " << &self.name).write_indented_list(&self.body, "{\n", "\n", "\n}")
    }
}
