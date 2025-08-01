// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        EnumDecl, MethodDecl, MixinIncludeDecl, OperatorDecl, SourceBuffer, StructDecl,
        StructEntry, ValDeclStatement,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for StructEntry {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::struct_entry);
        let content = p.into_inner().next().expect("needs an atom inside");
        match content.as_rule() {
            Rule::method_decl => {
                Self::Method(Box::new(MethodDecl::from_parse_tree(content, source)))
            }
            Rule::operator_decl => {
                Self::Operator(Box::new(OperatorDecl::from_parse_tree(content, source)))
            }
            Rule::val_decl_stmt => {
                Self::Variable(Box::new(ValDeclStatement::from_parse_tree(content, source)))
            }
            Rule::struct_decl => {
                Self::Struct(Box::new(StructDecl::from_parse_tree(content, source)))
            }
            Rule::enum_decl => Self::Enum(Box::new(EnumDecl::from_parse_tree(content, source))),
            Rule::mixin_include_decl => {
                Self::MixinInclude(Box::new(MixinIncludeDecl::from_parse_tree(content, source)))
            }
            _ => panic!("invalid struct entry kind"),
        }
    }
}

impl PrettyPrintable for StructEntry {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        match self {
            Self::Method(m) => m.prettyprint(buffer),
            Self::Operator(o) => o.prettyprint(buffer),
            Self::Variable(v) => v.prettyprint(buffer << "type "),
            Self::Struct(s) => s.prettyprint(buffer),
            Self::Enum(e) => e.prettyprint(buffer),
            Self::MixinInclude(m) => m.prettyprint(buffer),
        }
    }
}
