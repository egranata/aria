// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        ArgumentList, DeclarationId, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for ArgumentList {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::arg_list);
        let loc = From::from(&p.as_span());
        let inner = p.into_inner();
        let names = inner
            .map(|e| DeclarationId::from_parse_tree(e, source))
            .collect::<Vec<_>>();
        Self {
            loc: source.pointer(loc),
            names,
        }
    }
}

impl PrettyPrintable for ArgumentList {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write_separated_list(&self.names, ",")
    }
}
