// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        ImportFromStatement, ImportPath, ImportTarget, SourceBuffer,
    },
    grammar::Rule,
};

impl Derive for ImportFromStatement {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::import_id_stmt);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let what = ImportTarget::from_parse_tree(inner.next().expect("need identifiers"), source);
        let from = ImportPath::from_parse_tree(inner.next().expect("need a path"), source);
        Self {
            loc: source.pointer(loc),
            what,
            from,
        }
    }
}

impl PrettyPrintable for ImportFromStatement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "import " << &self.what << " from " << &self.from << ";"
    }
}
