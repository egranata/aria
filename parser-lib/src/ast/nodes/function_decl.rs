// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        ArgumentList, CodeBlock, Expression, FunctionDecl, Identifier, ReturnStatement,
        SourceBuffer, Statement,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for FunctionDecl {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::function_decl);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let name = Identifier::from_parse_tree(inner.next().expect("need identifier"), source);
        let p = inner.peek().unwrap();
        let args = if p.as_rule() == Rule::arg_list {
            let p = inner.next().unwrap();
            ArgumentList::from_parse_tree(p, source)
        } else {
            ArgumentList::empty(source.pointer(loc))
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
                    _ => panic!("Unexpected rule for inner function body: {i:?}"),
                }
            }
            _ => panic!("Unexpected rule for function body: {b:?}"),
        };
        Self {
            loc: source.pointer(loc),
            name,
            args,
            body,
        }
    }
}

impl PrettyPrintable for FunctionDecl {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "func " << &self.name << " (" << &self.args << ") " << &self.body
    }
}
