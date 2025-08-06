// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        Expression, ReturnStatement, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for ReturnStatement {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::return_stmt);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let val = inner.next().map(|p| Expression::from_parse_tree(p, source));
        Self {
            loc: source.pointer(loc),
            val,
        }
    }
}

impl PrettyPrintable for ReturnStatement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        let mut buffer = buffer << "return";
        if let Some(val) = &self.val {
            buffer = buffer << " " << val;
        }
        buffer << ";"
    }
}
