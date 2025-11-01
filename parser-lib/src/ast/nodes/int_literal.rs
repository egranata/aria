// SPDX-License-Identifier: Apache-2.0
use crate::ast::{
    IntLiteral,
    derive::Derive,
    prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
};

impl Derive for IntLiteral {
    fn from_parse_tree(
        p: pest::iterators::Pair<'_, crate::grammar::Rule>,
        source: &crate::ast::SourceBuffer,
    ) -> Self {
        assert!(p.as_rule() == crate::grammar::Rule::int_literal);
        let loc = From::from(&p.as_span());
        let val = p.as_str().to_owned();
        Self {
            loc: source.pointer(loc),
            val,
        }
    }
}

impl PrettyPrintable for IntLiteral {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write(&self.val)
    }
}
