// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        FloatLiteral, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for FloatLiteral {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::fp_literal);
        let loc = From::from(&p.as_span());
        Self {
            loc: source.pointer(loc),
            val: p.as_str().to_owned(),
        }
    }
}

impl PrettyPrintable for FloatLiteral {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write(&self.val)
    }
}
