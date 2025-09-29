// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        PostfixTermSigil, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for PostfixTermSigil {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::postfix_term_sigil);
        let loc = From::from(&p.as_span());
        let sigil = p.as_str().strip_prefix("@").unwrap().to_string();
        Self {
            loc: source.pointer(loc),
            sigil,
        }
    }
}

impl PrettyPrintable for PostfixTermSigil {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write(&self.sigil)
    }
}
