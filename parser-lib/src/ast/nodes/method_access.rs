// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        MethodAccess, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for MethodAccess {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, _: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::method_access);
        match p.as_str() {
            "instance" => Self::Instance,
            "type" => Self::Type,
            _ => panic!("instance or type expected"),
        }
    }
}

impl PrettyPrintable for MethodAccess {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer
            << match self {
                MethodAccess::Instance => "instance",
                MethodAccess::Type => "type",
            }
    }
}
