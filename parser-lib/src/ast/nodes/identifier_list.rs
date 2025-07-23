// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        Identifier, IdentifierList, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for IdentifierList {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::ident_list);
        let loc = From::from(&p.as_span());
        let inner = p.into_inner();
        let identifiers = inner
            .map(|e| Identifier::from_parse_tree(e, source))
            .collect::<Vec<_>>();
        Self {
            loc: source.pointer(loc),
            identifiers,
        }
    }
}

impl PrettyPrintable for IdentifierList {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write_separated_list(&self.identifiers, ",")
    }
}
