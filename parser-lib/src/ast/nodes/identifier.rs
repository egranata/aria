// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        Identifier, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for Identifier {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::identifier);
        let loc = From::from(&p.as_span());
        Self {
            loc: source.pointer(loc),
            value: p.as_str().to_owned(),
        }
    }
}

impl PrettyPrintable for Identifier {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write(&self.value)
    }
}
