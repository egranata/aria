// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        OperatorSymbol, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for OperatorSymbol {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, _: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::operator_symbol);
        match p.as_str() {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "u-" => Self::UnaryMinus,
            "*" => Self::Star,
            "/" => Self::Slash,
            "%" => Self::Percent,
            "<<" => Self::LeftShift,
            ">>" => Self::RightShift,
            "==" => Self::Equals,
            "<=" => Self::LessThanEqual,
            ">=" => Self::GreaterThanEqual,
            "<" => Self::LessThan,
            ">" => Self::GreaterThan,
            "&" => Self::BitwiseAnd,
            "|" => Self::BitwiseOr,
            "^" => Self::BitwiseXor,
            "()" => Self::Call,
            "[]" => Self::GetSquareBrackets,
            "[]=" => Self::SetSquareBrackets,
            _ => panic!("valid operator symbol expected, found: {}", p.as_str()),
        }
    }
}

impl PrettyPrintable for OperatorSymbol {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write(match self {
            Self::Plus => "+",
            Self::Minus => "-",
            Self::UnaryMinus => "u-",
            Self::Star => "*",
            Self::Slash => "/",
            Self::Percent => "%",
            Self::LeftShift => "<<",
            Self::RightShift => ">>",
            Self::LessThanEqual => "<=",
            Self::GreaterThanEqual => ">=",
            Self::LessThan => "<",
            Self::GreaterThan => ">",
            Self::Equals => "==",
            Self::BitwiseAnd => "&",
            Self::BitwiseOr => "|",
            Self::BitwiseXor => "^",
            Self::Call => "()",
            Self::GetSquareBrackets => "[]",
            Self::SetSquareBrackets => "[]=",
        })
    }
}
