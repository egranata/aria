// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        ImportPath, ImportStatement, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for ImportStatement {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::import_stmt);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let what = ImportPath::from_parse_tree(inner.next().expect("need a path"), source);
        Self {
            loc: source.pointer(loc),
            what,
        }
    }
}

impl PrettyPrintable for ImportStatement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "import " << &self.what << ";"
    }
}
