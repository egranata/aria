// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        CodeBlock, SourceBuffer, Statement,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for CodeBlock {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::code_block);
        let loc = From::from(&p.as_span());
        let inner = p.into_inner();
        let entries = inner
            .map(|e| Statement::from_parse_tree(e, source))
            .collect::<Vec<_>>();
        Self {
            loc: source.pointer(loc),
            entries,
        }
    }
}

impl PrettyPrintable for CodeBlock {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write_indented_list(
            &self.entries,
            "{
",
            "
",
            "}",
        )
    }
}
