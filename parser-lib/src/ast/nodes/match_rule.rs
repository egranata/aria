// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        CodeBlock, MatchPattern, MatchRule, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for MatchRule {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::match_rule);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let mut patterns = vec![];
        let then = {
            loop {
                let next = inner.next().expect("need rules");
                match next.as_rule() {
                    Rule::match_pattern => {
                        patterns.push(MatchPattern::from_parse_tree(next, source));
                    }
                    Rule::code_block => {
                        break CodeBlock::from_parse_tree(next, source);
                    }
                    _ => panic!("invalid rule entry"),
                }
            }
        };
        Self {
            loc: source.pointer(loc),
            patterns,
            then,
        }
    }
}

impl PrettyPrintable for MatchRule {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write_separated_list(&self.patterns, " and ") << " => " << &self.then
    }
}
