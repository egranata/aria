// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        MethodDecl, MixinEntry, MixinIncludeDecl, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for MixinEntry {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::mixin_entry);
        let content = p.into_inner().next().expect("needs an atom inside");
        match content.as_rule() {
            Rule::method_decl => {
                Self::Method(Box::new(MethodDecl::from_parse_tree(content, source)))
            }
            Rule::mixin_include_decl => {
                Self::Include(Box::new(MixinIncludeDecl::from_parse_tree(content, source)))
            }
            _ => panic!("invalid mixin entry kind"),
        }
    }
}

impl PrettyPrintable for MixinEntry {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        match self {
            Self::Method(m) => m.prettyprint(buffer),
            Self::Include(m) => m.prettyprint(buffer),
        }
    }
}
