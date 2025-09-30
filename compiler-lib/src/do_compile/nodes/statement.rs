// SPDX-License-Identifier: Apache-2.0
use crate::do_compile::{CompilationResult, CompileNode, CompileParams};

impl<'a> CompileNode<'a> for aria_parser::ast::Statement {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        match self {
            Self::ValDeclStatement(l) => l.do_compile(params),
            Self::AssignStatement(a) => a.do_compile(params),
            Self::WriteOpEqStatement(w) => w.do_compile(params),
            Self::IfStatement(i) => i.do_compile(params),
            Self::MatchStatement(m) => m.do_compile(params),
            Self::WhileStatement(w) => w.do_compile(params),
            Self::ForStatement(f) => f.do_compile(params),
            Self::ReturnStatement(r) => r.do_compile(params),
            Self::ThrowStatement(t) => t.do_compile(params),
            Self::GuardBlock(g) => g.do_compile(params),
            Self::TryBlock(t) => t.do_compile(params),
            Self::AssertStatement(a) => a.do_compile(params),
            Self::CodeBlock(c) => c.do_compile(params),
            Self::ExpressionStatement(e) => e.do_compile(params),
            Self::BreakStatement(b) => b.do_compile(params),
            Self::ContinueStatement(c) => c.do_compile(params),
            Self::StructDecl(s) => s.do_compile(params),
            Self::EnumDecl(e) => e.do_compile(params),
            Self::FunctionDecl(f) => {
                let f_scope = params.scope.closure(params.writer.get_current_block());
                let mut f_params = CompileParams {
                    module: params.module,
                    scope: &f_scope,
                    writer: params.writer,
                    cflow: params.cflow,
                    options: params.options,
                };
                f.do_compile(&mut f_params)
            }
        }
    }
}
