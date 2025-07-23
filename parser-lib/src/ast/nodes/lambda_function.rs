// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        ArgumentList, LambaBody, LambdaFunction, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for LambdaFunction {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::lambda_f);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let args = ArgumentList::from_parse_tree(inner.next().expect("need arguments"), source);
        let body = LambaBody::from_parse_tree(inner.next().expect("need body"), source);
        Self {
            loc: source.pointer(loc),
            args,
            body: Box::new(body),
        }
    }
}

impl PrettyPrintable for LambdaFunction {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "|" << &self.args << "| => " << &self.body
    }
}
