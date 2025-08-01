// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        ArgumentList, CodeBlock, OperatorDecl, OperatorSymbol, SourceBuffer,
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
        let body = CodeBlock::from_parse_tree(inner.next().expect("need body"), source);
        Self {
            loc: source.pointer(loc),
            reverse,
            symbol,
            args,
            vararg,
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
            << if self.vararg { "..." } else { "" }
            << ") "
            << &self.body
    }
}
