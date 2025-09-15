// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        PostfixTermTryProtocol, SourceBuffer, TryProtocolMode,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for PostfixTermTryProtocol {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::postfix_term_try_protocol);
        let loc = From::from(&p.as_span());
        let mode = match p.as_str() {
            "??" => TryProtocolMode::Return,
            "!!" => TryProtocolMode::Assert,
            _ => panic!("?? or !! expected"),
        };

        Self {
            loc: source.pointer(loc),
            mode,
        }
    }
}

impl PrettyPrintable for PostfixTermTryProtocol {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer
            << match self.mode {
                TryProtocolMode::Assert => "!!",
                TryProtocolMode::Return => "??",
            }
    }
}
