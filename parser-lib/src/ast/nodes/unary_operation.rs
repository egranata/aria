// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        PostfixRvalue, SourceBuffer, UnaryOperation, UnarySymbol,
    },
    grammar::Rule,
};

impl Derive for UnaryOperation {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::unary);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        if inner.len() == 1 {
            let postfix = inner.next().expect("need postfix");
            let postfix = PostfixRvalue::from_parse_tree(postfix, source);
            Self {
                loc: source.pointer(loc),
                operand: None,
                postfix,
            }
        } else if inner.len() == 2 {
            let operand = inner.next().expect("need operand");
            let operand = UnarySymbol::from_parse_tree(operand, source);
            let postfix = inner.next().expect("need postfix");
            let postfix = PostfixRvalue::from_parse_tree(postfix, source);
            Self {
                loc: source.pointer(loc),
                operand: Some(operand),
                postfix,
            }
        } else {
            panic!("unexpected unary operation structure");
        }
    }
}

impl PrettyPrintable for UnaryOperation {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << &self.operand << &self.postfix
    }
}
