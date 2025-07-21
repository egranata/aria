// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        AddEqSymbol, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for AddEqSymbol {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, _: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::add_op_eq);
        match p.as_str() {
            "+=" => Self::PlusEq,
            "-=" => Self::MinusEq,
            "*=" => Self::StarEq,
            "/=" => Self::SlashEq,
            "%=" => Self::PercentEq,
            _ => panic!("operator expected"),
        }
    }
}

impl PrettyPrintable for AddEqSymbol {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write(match self {
            Self::PlusEq => "+=",
            Self::MinusEq => "-=",
            Self::StarEq => "*=",
            Self::SlashEq => "/=",
            Self::PercentEq => "%=",
        })
    }
}
