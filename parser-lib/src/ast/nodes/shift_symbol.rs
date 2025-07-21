// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        ShiftSymbol, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for ShiftSymbol {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, _: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::shift_op);
        match p.as_str() {
            "<<" => Self::Leftward,
            ">>" => Self::Rightward,
            _ => panic!("<< or >> expected"),
        }
    }
}

impl PrettyPrintable for ShiftSymbol {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write(match self {
            Self::Leftward => "<<",
            Self::Rightward => ">>",
        })
    }
}
