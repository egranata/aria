// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        MulSymbol, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for MulSymbol {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, _: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::mul_op);
        match p.as_str() {
            "*" => MulSymbol::Star,
            "/" => MulSymbol::Slash,
            "%" => MulSymbol::Percent,
            _ => panic!("one of *, / or % expected"),
        }
    }
}

impl PrettyPrintable for MulSymbol {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write(match self {
            Self::Star => "*",
            Self::Slash => "/",
            Self::Percent => "%",
        })
    }
}
