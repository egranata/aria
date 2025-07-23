// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        CompOperation, LogOperation, LogSymbol, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for LogOperation {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::log);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        if inner.len() == 1 {
            let left = CompOperation::from_parse_tree(inner.peek().expect("need a mul"), source);
            Self {
                loc: source.pointer(loc),
                left,
                right: vec![],
            }
        } else if inner.len() > 0 {
            let left =
                CompOperation::from_parse_tree(inner.next().expect("need a left mul"), source);
            let mut right = vec![];
            loop {
                let op = inner.next();
                if op.is_none() {
                    break;
                };
                let op = LogSymbol::from_parse_tree(op.unwrap(), source);
                let atom = CompOperation::from_parse_tree(
                    inner.next().expect("add needs a right hand side"),
                    source,
                );
                right.push((op, atom));
            }
            Self {
                loc: source.pointer(loc),
                left,
                right,
            }
        } else {
            panic!("add does not contain")
        }
    }
}

impl PrettyPrintable for LogOperation {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        let mut this = self.left.prettyprint(buffer);
        for (op, atom) in &self.right {
            this = this << op << atom;
        }
        this
    }
}
