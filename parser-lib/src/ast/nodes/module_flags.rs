// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        ModuleFlag, ModuleFlags, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for ModuleFlags {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::module_flags);
        let inner = p.into_inner();
        let mut flags = vec![];
        for entry in inner {
            match entry.as_rule() {
                Rule::module_flag => {
                    flags.push(ModuleFlag::from_parse_tree(entry, source));
                }
                _ => panic!("invalid module flag"),
            }
        }
        Self { flags }
    }
}

impl PrettyPrintable for ModuleFlags {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write_separated_list(&self.flags, "\n")
    }
}
