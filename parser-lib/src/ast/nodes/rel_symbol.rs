// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        RelSymbol, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for RelSymbol {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, _: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::rel_op);
        match p.as_str() {
            "<" => RelSymbol::Less,
            "<=" => RelSymbol::LessEqual,
            ">" => RelSymbol::Greater,
            ">=" => RelSymbol::GreaterEqual,
            _ => panic!("<=> expected"),
        }
    }
}

impl PrettyPrintable for RelSymbol {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write(match self {
            Self::Less => "<",
            Self::LessEqual => "<=",
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
        })
    }
}
