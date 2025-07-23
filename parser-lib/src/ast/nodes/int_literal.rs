// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        IntLiteral, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for IntLiteral {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::int_literal);
        let loc = From::from(&p.as_span());
        let inp_str = p.as_str().to_owned().replace('_', "");

        let val = if let Some(hex_str) = inp_str.strip_prefix("0x") {
            i64::from_str_radix(hex_str, 16)
        } else if let Some(bin_str) = inp_str.strip_prefix("0b") {
            i64::from_str_radix(bin_str, 2)
        } else if let Some(oct_str) = inp_str.strip_prefix("0o") {
            i64::from_str_radix(oct_str, 8)
        } else {
            inp_str.parse::<i64>()
        }
        .expect("invalid literal");

        Self {
            loc: source.pointer(loc),
            val,
        }
    }
}

impl PrettyPrintable for IntLiteral {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << &self.val
    }
}
