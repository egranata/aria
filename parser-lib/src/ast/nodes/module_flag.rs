// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        Identifier, ModuleFlag, SourceBuffer, StringLiteral,
    },
    grammar::Rule,
};

impl Derive for ModuleFlag {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::module_flag);
        let mut inner = p.into_inner();
        let flag = Identifier::from_parse_tree(inner.next().expect("need flag"), source);
        match flag.value.as_str() {
            "no_std" => Self::NoStandardLibrary,
            "uses_dylib" => {
                let path = StringLiteral::from_parse_tree(inner.next().expect("need path"), source);
                Self::UsesDylib(path.value)
            }
            _ => panic!("unknown module flag"),
        }
    }
}

impl PrettyPrintable for ModuleFlag {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        match self {
            ModuleFlag::NoStandardLibrary => buffer << "flag: no_std;",
            ModuleFlag::UsesDylib(dylib) => buffer << "flag: uses_dylib(" << dylib.as_str() << ");",
        }
    }
}
