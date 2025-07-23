// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        CompSymbol, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for CompSymbol {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, _: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::comp_op);
        match p.as_str() {
            "==" => Self::Equal,
            "!=" => Self::NotEqual,
            "isa" => Self::Isa,
            _ => panic!("* or / expected"),
        }
    }
}

impl PrettyPrintable for CompSymbol {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write(match self {
            Self::Equal => "==",
            Self::NotEqual => "!=",
            Self::Isa => "isa",
        })
    }
}
