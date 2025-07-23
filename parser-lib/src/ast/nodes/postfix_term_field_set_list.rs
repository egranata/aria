// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        PostfixTermFieldSet, PostfixTermFieldSetList, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for PostfixTermFieldSetList {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::postfix_field_set_list);
        let loc = From::from(&p.as_span());
        let inner = p.into_inner();
        let terms = inner
            .map(|i| PostfixTermFieldSet::from_parse_tree(i, source))
            .collect();
        Self {
            loc: source.pointer(loc),
            terms,
        }
    }
}

impl PrettyPrintable for PostfixTermFieldSetList {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write_separated_list(&self.terms, ", ")
    }
}
