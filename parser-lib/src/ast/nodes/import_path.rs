// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        Identifier, ImportPath, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for ImportPath {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::import_path);
        let loc = From::from(&p.as_span());
        let inner = p.into_inner();
        let entries = inner
            .map(|x| Identifier::from_parse_tree(x, source))
            .collect();
        Self {
            loc: source.pointer(loc),
            entries,
        }
    }
}

impl PrettyPrintable for ImportPath {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write_separated_list(&self.entries, ".")
    }
}
