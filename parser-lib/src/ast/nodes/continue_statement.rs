// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        ContinueStatement, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for ContinueStatement {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::continue_stmt);
        let loc = From::from(&p.as_span());
        Self {
            loc: source.pointer(loc),
        }
    }
}

impl PrettyPrintable for ContinueStatement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "continue;"
    }
}
