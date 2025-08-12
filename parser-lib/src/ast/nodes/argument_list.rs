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
        let mut names = vec![];
        let mut vararg = false;
        for e in inner {
            if e.as_rule() == Rule::vararg_marker {
                vararg = true;
            } else if e.as_rule() == Rule::decl_id {
                names.push(DeclarationId::from_parse_tree(e, source));
            } else {
                panic!("Unexpected rule in argument list: {:?}", e.as_rule());
            }
        }
        Self {
            loc: source.pointer(loc),
            names,
            vararg,
        }
    }
}

impl PrettyPrintable for ArgumentList {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        let buffer = buffer.write_separated_list(&self.names, ",");
        if self.vararg { buffer << "..." } else { buffer }
    }
}
