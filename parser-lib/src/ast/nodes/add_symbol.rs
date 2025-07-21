// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        AddSymbol, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for AddSymbol {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, _: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::add_op);
        match p.as_str() {
            "+" => Self::Plus,
            "-" => Self::Minus,
            _ => panic!("+ or - expected"),
        }
    }
}

impl PrettyPrintable for AddSymbol {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write(match self {
            Self::Plus => "+",
            Self::Minus => "-",
        })
    }
}
