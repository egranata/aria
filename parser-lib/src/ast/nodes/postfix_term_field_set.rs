// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        Expression, Identifier, PostfixTermFieldSet, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for PostfixTermFieldSet {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::postfix_field_set);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let id = Identifier::from_parse_tree(inner.next().expect("postfix need id"), source);
        let val = inner.next().map(|p| Expression::from_parse_tree(p, source));
        Self {
            loc: source.pointer(loc),
            id,
            val,
        }
    }
}

impl PrettyPrintable for PostfixTermFieldSet {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        let buffer = buffer << "." << &self.id;
        if let Some(val) = &self.val {
            buffer << " = " << val
        } else {
            buffer
        }
    }
}
