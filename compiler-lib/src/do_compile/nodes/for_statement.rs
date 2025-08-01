// SPDX-License-Identifier: Apache-2.0
use aria_parser::ast::{
    BreakStatement, CodeBlock, DeclarationId, ElsePiece, Expression, Identifier, IfCondPiece,
    IfPiece, IfStatement, ParenExpression, PostfixExpression, Primary, Statement, ValDeclStatement,
    WhileStatement,
};

use crate::do_compile::{CompilationResult, CompileNode, CompileParams};

impl<'a> CompileNode<'a> for aria_parser::ast::ForStatement {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        let iter_name = format!("__for__iter__{}", self.loc.clone());
        let iter_name_ident = Identifier {
            loc: self.id.loc.clone(),
            value: iter_name,
        };
        let fetch_iter_expr = Expression::from(&PostfixExpression::method_call(
            &Primary::ParenExpression(ParenExpression::from(&self.expr)),
            "iterator",
            &[],
        ));
        let fetch_iter_val = Statement::ValDeclStatement(ValDeclStatement {
            loc: self.loc.clone(),
            id: DeclarationId::from(&iter_name_ident),
            val: fetch_iter_expr,
        });
        let true_cond = Expression::from(&Identifier {
            loc: self.loc.clone(),
            value: "true".to_owned(),
        });
        let val_next_name = format!("__for__next__{}", self.loc.clone());
        let val_next_ident = Identifier {
            loc: self.id.loc.clone(),
            value: val_next_name,
        };
        let fetch_next_expr = Expression::from(&PostfixExpression::method_call(
            &Primary::Identifier(iter_name_ident),
            "next",
            &[],
        ));
        let fetch_next_val = Statement::ValDeclStatement(ValDeclStatement {
            loc: self.loc.clone(),
            id: DeclarationId::from(&val_next_ident),
            val: fetch_next_expr,
        });
        let check_done_expr = Expression::from(&PostfixExpression::attrib_read(
            &Primary::Identifier(val_next_ident.clone()),
            "done",
        ));
        let if_done_blk = CodeBlock {
            loc: self.loc.clone(),
            entries: vec![Statement::BreakStatement(BreakStatement {
                loc: self.loc.clone(),
            })],
        };
        let read_next_expr = Expression::from(&PostfixExpression::attrib_read(
            &Primary::Identifier(val_next_ident.clone()),
            "value",
        ));
        let assign_to_loop_val = Statement::ValDeclStatement(ValDeclStatement {
            loc: self.id.loc.clone(),
            id: DeclarationId::from(&self.id),
            val: read_next_expr,
        });
        let if_more_blk = CodeBlock {
            loc: self.loc.clone(),
            entries: vec![assign_to_loop_val, Statement::CodeBlock(self.then.clone())],
        };
        let check_stmt = Statement::IfStatement(IfStatement {
            loc: self.loc.clone(),
            iff: IfPiece {
                content: IfCondPiece {
                    loc: self.loc.clone(),
                    expression: Box::new(check_done_expr),
                    then: if_done_blk,
                },
            },
            elsif: vec![],
            els: Some(ElsePiece {
                loc: self.loc.clone(),
                then: if_more_blk,
            }),
        });
        let while_body = CodeBlock {
            loc: self.loc.clone(),
            entries: vec![fetch_next_val, check_stmt],
        };
        let w = Statement::WhileStatement(WhileStatement {
            loc: self.loc.clone(),
            cond: true_cond,
            then: while_body,
            els: self.els.clone(),
        });
        let blk = CodeBlock {
            loc: self.loc.clone(),
            entries: vec![fetch_iter_val, w],
        };
        blk.do_compile(params)
    }
}
