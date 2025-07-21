// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        ModuleFlags, ParsedModule, SourceBuffer, TopLevelEntry,
    },
    grammar::Rule,
};

impl Derive for ParsedModule {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::module);
        let loc = From::from(&p.as_span());
        let inner = p.into_inner();
        let mut flags = ModuleFlags::empty();
        let mut entries = vec![];
        for entry in inner {
            match entry.as_rule() {
                Rule::module_flags => flags = ModuleFlags::from_parse_tree(entry, source),
                Rule::top_level_entry => {
                    entries.push(TopLevelEntry::from_parse_tree(entry, source))
                }
                _ => {}
            }
        }
        Self {
            loc: source.pointer(loc),
            flags,
            entries,
        }
    }
}

impl PrettyPrintable for ParsedModule {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        self.flags
            .prettyprint(buffer)
            .write_separated_list(&self.entries, "\n")
    }
}
