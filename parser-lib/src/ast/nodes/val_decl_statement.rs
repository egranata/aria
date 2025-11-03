// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        SourceBuffer, ValDeclEntry, ValDeclStatement,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for ValDeclStatement {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::val_decl_stmt);
        let loc = From::from(&p.as_span());
        let decls = p
            .into_inner()
            .map(|i| ValDeclEntry::from_parse_tree(i, source))
            .collect::<Vec<_>>();
        Self {
            loc: source.pointer(loc),
            decls,
        }
    }
}

impl PrettyPrintable for ValDeclStatement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        (buffer << "val ").write_separated_list(&self.decls, ", ") << ";"
    }
}
