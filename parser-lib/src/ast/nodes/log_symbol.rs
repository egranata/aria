// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        LogSymbol, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for LogSymbol {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, _: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::log_op);
        match p.as_str() {
            "&" => Self::Ampersand,
            "^" => Self::Caret,
            "|" => Self::Pipe,
            "&&" => Self::DoubleAmpersand,
            "||" => Self::DoublePipe,
            _ => panic!("&^| expected"),
        }
    }
}

impl PrettyPrintable for LogSymbol {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write(match self {
            Self::Ampersand => "&",
            Self::DoubleAmpersand => "&&",
            Self::Caret => "^",
            Self::Pipe => "|",
            Self::DoublePipe => "||",
        })
    }
}
