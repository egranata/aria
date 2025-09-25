// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        DeclarationId, Identifier, MatchPatternEnumCase, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for MatchPatternEnumCase {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::match_pattern_enum_case);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let case = Identifier::from_parse_tree(inner.next().expect("need expression"), source);
        let payload = inner
            .next()
            .map(|next| DeclarationId::from_parse_tree(next, source));
        Self {
            loc: source.pointer(loc),
            case,
            payload,
        }
    }
}

impl PrettyPrintable for MatchPatternEnumCase {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        let buffer = buffer << "case " << &self.case;
        if let Some(p) = &self.payload {
            buffer << "(" << p << ")"
        } else {
            buffer
        }
    }
}
