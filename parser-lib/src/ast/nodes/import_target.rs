// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        IdentifierList, ImportTarget, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for ImportTarget {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::import_target);
        let mut inner = p.into_inner();
        let tgt = inner.next().expect("need an import target");
        match tgt.as_rule() {
            Rule::ident_list => Self::IdentifierList(IdentifierList::from_parse_tree(tgt, source)),
            Rule::import_all => Self::All,
            _ => panic!("invalid import target"),
        }
    }
}

impl PrettyPrintable for ImportTarget {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        match self {
            ImportTarget::IdentifierList(il) => il.prettyprint(buffer),
            ImportTarget::All => buffer << "*",
        }
    }
}
