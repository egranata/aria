// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        SourceBuffer, UnarySymbol,
    },
    grammar::Rule,
};

impl Derive for UnarySymbol {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, _: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::unary_op);
        match p.as_str() {
            "!" => Self::Exclamation,
            "-" => Self::Minus,
            _ => panic!("! or - expected"),
        }
    }
}

impl PrettyPrintable for UnarySymbol {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write(match self {
            Self::Exclamation => "!",
            Self::Minus => "-",
        })
    }
}
