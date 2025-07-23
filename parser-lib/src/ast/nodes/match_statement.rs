// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        ElsePiece, Expression, MatchRule, MatchStatement, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for MatchStatement {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::match_stmt);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let expr = Expression::from_parse_tree(inner.next().expect("need expression"), source);
        let mut rules = vec![];
        let els = {
            loop {
                let next = match inner.next() {
                    Some(x) => x,
                    None => {
                        break None;
                    }
                };
                match next.as_rule() {
                    Rule::match_rule => {
                        rules.push(MatchRule::from_parse_tree(next, source));
                    }
                    Rule::else_piece => {
                        break Some(ElsePiece::from_parse_tree(next, source));
                    }
                    _ => panic!("invalid rule entry"),
                }
            }
        };
        Self {
            loc: source.pointer(loc),
            expr,
            rules,
            els,
        }
    }
}

impl PrettyPrintable for MatchStatement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        let buffer =
            (buffer << "match " << &self.expr).write_indented_list(&self.rules, "{\n", "\n", "");
        if let Some(e) = &self.els {
            buffer << "\n else " << e << "\n}"
        } else {
            buffer << "\n}"
        }
    }
}
