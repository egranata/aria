// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        ExpressionList, PostfixTermCall, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for PostfixTermCall {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::postfix_term_call);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let args = inner.find(|val| val.as_rule() == Rule::expr_list);
        let args = if let Some(el) = args {
            ExpressionList::from_parse_tree(el, source)
        } else {
            ExpressionList::empty(source.pointer(loc))
        };
        Self {
            loc: source.pointer(loc),
            args,
        }
    }
}

impl PrettyPrintable for PostfixTermCall {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "(" << &self.args << ")"
    }
}
