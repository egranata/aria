// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        ArgumentList, CodeBlock, FunctionDecl, Identifier, SourceBuffer,
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
        let vararg = if inner.peek().unwrap().as_rule() == Rule::vararg_marker {
            let _ = inner.next();
            true
        } else {
            false
        };
        let body = CodeBlock::from_parse_tree(inner.next().expect("need body"), source);
        Self {
            loc: source.pointer(loc),
            name,
            args,
            vararg,
            body,
        }
    }
}

impl PrettyPrintable for FunctionDecl {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer
            << "func "
            << &self.name
            << " ("
            << &self.args
            << if self.vararg { "..." } else { "" }
            << ") "
            << &self.body
    }
}
