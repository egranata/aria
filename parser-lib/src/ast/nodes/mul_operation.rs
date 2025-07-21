// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        MulOperation, MulSymbol, SourceBuffer, UnaryOperation,
    },
    grammar::Rule,
};

impl Derive for MulOperation {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::mul);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let left = UnaryOperation::from_parse_tree(inner.peek().expect("need an atom"), source);
        if inner.len() == 1 {
            Self {
                loc: source.pointer(loc),
                left,
                right: vec![],
            }
        } else if inner.len() > 0 {
            let _ = inner.next();
            let mut right = vec![];
            loop {
                let op = inner.next();
                if op.is_none() {
                    break;
                };
                let op = MulSymbol::from_parse_tree(op.unwrap(), source);
                let atom = UnaryOperation::from_parse_tree(
                    inner.next().expect("mul needs a right hand side"),
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
            panic!("mul does not contain")
        }
    }
}

impl PrettyPrintable for MulOperation {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        let mut this = self.left.prettyprint(buffer);
        for (op, atom) in &self.right {
            this = this << op << atom;
        }
        this
    }
}
