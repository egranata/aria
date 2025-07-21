// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        AssertStatement, AssignStatement, EnumDecl, ExtensionDecl, FunctionDecl,
        ImportFromStatement, ImportStatement, MixinDecl, StructDecl, TopLevelEntry,
        ValDeclStatement,
    },
    gen_from_options,
};

impl Derive for TopLevelEntry {
    gen_from_options!(
        top_level_entry;
        (assert_stmt, AssertStatement),
        (enum_decl, EnumDecl),
        (extension_decl, ExtensionDecl),
        (function_decl, FunctionDecl),
        (import_id_stmt, ImportFromStatement),
        (import_stmt, ImportStatement),
        (mixin_decl, MixinDecl),
        (struct_decl, StructDecl),
        (val_decl_stmt, ValDeclStatement),
        (val_write_stmt, AssignStatement),
    );
}

impl PrettyPrintable for TopLevelEntry {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        match self {
            Self::ValDeclStatement(v) => v.prettyprint(buffer),
            Self::AssignStatement(a) => a.prettyprint(buffer),
            Self::FunctionDecl(f) => f.prettyprint(buffer),
            Self::StructDecl(s) => s.prettyprint(buffer),
            Self::MixinDecl(m) => m.prettyprint(buffer),
            Self::EnumDecl(e) => e.prettyprint(buffer),
            Self::ExtensionDecl(e) => e.prettyprint(buffer),
            Self::AssertStatement(a) => a.prettyprint(buffer),
            Self::ImportStatement(i) => i.prettyprint(buffer),
            Self::ImportFromStatement(i) => i.prettyprint(buffer),
        }
    }
}
