// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        AssertStatement, AssignStatement, BreakStatement, CodeBlock, ContinueStatement, EnumDecl,
        ExpressionStatement, ForStatement, GuardBlock, IfStatement, MatchStatement,
        ReturnStatement, Statement, StructDecl, ThrowStatement, TryBlock, ValDeclStatement,
        WhileStatement, WriteOpEqStatement,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_options,
};

impl Derive for Statement {
    gen_from_options!(
        statement;
        (assert_stmt, AssertStatement),
        (break_stmt, BreakStatement),
        (code_block, CodeBlock),
        (continue_stmt, ContinueStatement),
        (expr_stmt, ExpressionStatement),
        (for_stmt, ForStatement),
        (guard_block, GuardBlock),
        (if_stmt, IfStatement),
        (match_stmt, MatchStatement),
        (return_stmt, ReturnStatement),
        (throw_stmt, ThrowStatement),
        (try_block, TryBlock),
        (val_add_eq_write, WriteOpEqStatement),
        (val_decl_stmt, ValDeclStatement),
        (val_write_stmt, AssignStatement),
        (while_stmt, WhileStatement),
        (struct_decl, StructDecl),
        (enum_decl, EnumDecl),
    );
}

impl PrettyPrintable for Statement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        match self {
            Self::ValDeclStatement(v) => v.prettyprint(buffer),
            Self::AssignStatement(a) => a.prettyprint(buffer),
            Self::WriteOpEqStatement(w) => w.prettyprint(buffer),
            Self::IfStatement(i) => i.prettyprint(buffer),
            Self::MatchStatement(m) => m.prettyprint(buffer),
            Self::WhileStatement(w) => w.prettyprint(buffer),
            Self::ForStatement(f) => f.prettyprint(buffer),
            Self::CodeBlock(c) => c.prettyprint(buffer),
            Self::ReturnStatement(r) => r.prettyprint(buffer),
            Self::ThrowStatement(t) => t.prettyprint(buffer),
            Self::GuardBlock(g) => g.prettyprint(buffer),
            Self::TryBlock(t) => t.prettyprint(buffer),
            Self::AssertStatement(a) => a.prettyprint(buffer),
            Self::ExpressionStatement(e) => e.prettyprint(buffer),
            Self::BreakStatement(b) => b.prettyprint(buffer),
            Self::ContinueStatement(c) => c.prettyprint(buffer),
            Self::StructDecl(s) => s.prettyprint(buffer),
            Self::EnumDecl(e) => e.prettyprint(buffer),
        }
    }
}
