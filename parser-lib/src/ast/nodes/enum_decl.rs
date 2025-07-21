// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        EnumCaseDecl, EnumDecl, EnumDeclEntry, Identifier, SourceBuffer, StructEntry,
    },
    grammar::Rule,
};

impl Derive for EnumDecl {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::enum_decl);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let name = Identifier::from_parse_tree(inner.next().expect("need identifier"), source);
        let mut entries = vec![];
        for next in inner {
            if next.as_rule() != Rule::enum_decl_entry {
                panic!("invalid enum entry :{next}");
            }
            let inner_rule = next.into_inner().next().expect("need enum entry");
            match inner_rule.as_rule() {
                Rule::struct_entry => {
                    entries.push(EnumDeclEntry::StructEntry(StructEntry::from_parse_tree(
                        inner_rule, source,
                    )));
                }
                Rule::enum_case_decl => {
                    entries.push(EnumDeclEntry::EnumCaseDecl(EnumCaseDecl::from_parse_tree(
                        inner_rule, source,
                    )));
                }
                _ => panic!("invalid enum entry :{inner_rule}"),
            }
        }
        Self {
            loc: source.pointer(loc),
            name,
            body: entries,
        }
    }
}

impl PrettyPrintable for EnumDecl {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        (buffer << "enum " << &self.name).write_indented_list(&self.body, "{\n", "\n", "\n}")
    }
}
