// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        AssertStatement, AssignStatement, CodeBlock, EnumDecl, ExpressionStatement, ExtensionDecl,
        ForStatement, FunctionDecl, GuardBlock, IfStatement, ImportFromStatement, ImportStatement,
        MatchStatement, MixinDecl, StructDecl, TopLevelEntry, TryBlock, ValDeclStatement,
        WhileStatement, WriteOpEqStatement,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_options,
};

impl Derive for TopLevelEntry {
    gen_from_options!(
        top_level_entry;
        (assert_stmt, AssertStatement),
        (enum_decl, EnumDecl),
        (expr_stmt, ExpressionStatement),
        (extension_decl, ExtensionDecl),
        (function_decl, FunctionDecl),
        (import_id_stmt, ImportFromStatement),
        (import_stmt, ImportStatement),
        (mixin_decl, MixinDecl),
        (struct_decl, StructDecl),
        (val_decl_stmt, ValDeclStatement),
        (val_write_stmt, AssignStatement),
        (if_stmt, IfStatement),
        (match_stmt, MatchStatement),
        (while_stmt, WhileStatement),
        (for_stmt, ForStatement),
        (val_add_eq_write, WriteOpEqStatement),
        (guard_block, GuardBlock),
        (try_block, TryBlock),
        (code_block, CodeBlock),
    );
}

impl PrettyPrintable for TopLevelEntry {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        match self {
            Self::ValDeclStatement(v) => v.prettyprint(buffer),
            Self::WriteOpEqStatement(w) => w.prettyprint(buffer),
            Self::AssignStatement(a) => a.prettyprint(buffer),
            Self::FunctionDecl(f) => f.prettyprint(buffer),
            Self::StructDecl(s) => s.prettyprint(buffer),
            Self::MixinDecl(m) => m.prettyprint(buffer),
            Self::EnumDecl(e) => e.prettyprint(buffer),
            Self::ExtensionDecl(e) => e.prettyprint(buffer),
            Self::AssertStatement(a) => a.prettyprint(buffer),
            Self::ExpressionStatement(e) => e.prettyprint(buffer),
            Self::ImportStatement(i) => i.prettyprint(buffer),
            Self::ImportFromStatement(i) => i.prettyprint(buffer),
            Self::IfStatement(i) => i.prettyprint(buffer),
            Self::MatchStatement(m) => m.prettyprint(buffer),
            Self::WhileStatement(w) => w.prettyprint(buffer),
            Self::ForStatement(f) => f.prettyprint(buffer),
            Self::CodeBlock(c) => c.prettyprint(buffer),
            Self::GuardBlock(g) => g.prettyprint(buffer),
            Self::TryBlock(t) => t.prettyprint(buffer),
        }
    }
}
