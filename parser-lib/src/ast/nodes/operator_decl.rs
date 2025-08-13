// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        ArgumentList, CodeBlock, Expression, OperatorDecl, OperatorSymbol, ReturnStatement,
        SourceBuffer, Statement,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for OperatorDecl {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::operator_decl);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let next = inner.peek().expect("expected next");
        let reverse = if next.as_rule() == Rule::operator_direction {
            let _ = inner.next();
            true
        } else {
            false
        };
        let symbol =
            OperatorSymbol::from_parse_tree(inner.next().expect("need operator symbol"), source);
        let next = inner.peek().unwrap();
        let args = if next.as_rule() == Rule::arg_list {
            let p = inner.next().unwrap();
            ArgumentList::from_parse_tree(p, source)
        } else {
            ArgumentList::empty(source.pointer(loc))
        };
        let vararg = if inner.peek().unwrap().as_rule() == Rule::vararg_marker {
            let _ = inner.next();
            true
        } else {
            false
        };
        let b = inner.next().expect("need body");
        let body = match b.as_rule() {
            Rule::code_block => CodeBlock::from_parse_tree(b, source),
            Rule::function_body => {
                let mut inner_body = b.into_inner();
                let i = inner_body.next().expect("need inner body part");
                match i.as_rule() {
                    Rule::code_block => CodeBlock::from_parse_tree(i, source),
                    Rule::expression => {
                        let expr = Expression::from_parse_tree(i, source);
                        let return_stmt = ReturnStatement {
                            loc: source.pointer(loc),
                            val: Some(expr),
                        };
                        CodeBlock {
                            loc: source.pointer(loc),
                            entries: vec![Statement::ReturnStatement(return_stmt)],
                        }
                    }
                    _ => panic!("Unexpected rule for inner function body: {:?}", i),
                }
            }
            _ => panic!("Unexpected rule for function body: {:?}", b),
        };
        Self {
            loc: source.pointer(loc),
            reverse,
            symbol,
            args,
            body,
        }
    }
}

impl PrettyPrintable for OperatorDecl {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer
            << if self.reverse {
                "reverse operator "
            } else {
                "operator "
            }
            << &self.symbol
            << " ("
            << &self.args
            << ") "
            << &self.body
    }
}
