// SPDX-License-Identifier: Apache-2.0
use aria_parser::ast::{
    AssignStatement, BreakStatement, CodeBlock, DeclarationId, ElsePiece, Expression, Identifier,
    IfCondPiece, IfPiece, IfStatement, MatchRule, MatchStatement, ParenExpression,
    PostfixExpression, PostfixRvalue, PostfixTerm, PostfixTermEnumCase, Primary, Statement,
    ThrowStatement, UnaryOperation, ValDeclStatement, WhileStatement,
};

use crate::do_compile::{CompilationResult, CompileNode, CompileParams};

macro_rules! val_decl_statement {
    ($loc:expr, $id:expr, $val:expr) => {
        Statement::ValDeclStatement(ValDeclStatement {
            loc: $loc,
            id: DeclarationId::from(&Identifier {
                loc: $loc,
                value: $id.clone(),
            }),
            val: $val,
        })
    };
}

impl<'a> CompileNode<'a> for aria_parser::ast::ForStatement {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        // the else block makes the logic here quite tricky and worth commenting
        // essentially it boils down to the following logic:

        // {
        //     val iter = <iterator expr>;
        //     val any_hit = false;
        //     while true {
        //         val next = iter.next();
        //         match next {
        //             case Some(x) => {
        //                 any_hit = true;
        //                 val x = next.value;
        //                 <body of the loop>
        //             }
        //             case None => {
        //                 if !any_hit {
        //                     <else clause if any>
        //                 }
        //                 break; # out of the while loop
        //             }
        //         }
        //     }
        // }

        // this is the iterator
        let iter_name_ident = Identifier {
            loc: self.loc.clone(),
            value: format!("__for__iter__{}", self.id.value),
        };

        // this is the next value from the iterator (the Maybe, not the actual object)
        let val_next_ident = Identifier {
            loc: self.loc.clone(),
            value: format!("__for__next__{}", self.id.value),
        };

        // this becomes true when the for {} body is executed at least once
        let any_hit_ident = Identifier {
            loc: self.loc.clone(),
            value: format!("__for__any_hit__{}", self.id.value),
        };

        let fetch_iter_val = val_decl_statement!(
            self.loc.clone(),
            iter_name_ident.value,
            Expression::from(&PostfixExpression::method_call(
                &Primary::ParenExpression(ParenExpression::from(&self.expr)),
                "iterator",
                &[]
            ))
        );

        let any_hit_val = val_decl_statement!(
            self.loc.clone(),
            any_hit_ident.value,
            Expression::from(&Identifier {
                loc: self.loc.clone(),
                value: "false".to_owned(),
            })
        );

        let true_cond = Expression::from(&Identifier {
            loc: self.loc.clone(),
            value: "true".to_owned(),
        });

        // val __for__next__ = __for__iter__.next();
        let fetch_next_val = val_decl_statement!(
            self.loc.clone(),
            val_next_ident.value,
            Expression::from(&PostfixExpression::method_call(
                &Primary::Identifier(iter_name_ident.clone()),
                "next",
                &[]
            ))
        );

        // !__for__any_hit
        let check_any_hit_expr = Expression::from(&UnaryOperation {
            loc: self.loc.clone(),
            operand: Some(aria_parser::ast::UnarySymbol::Exclamation),
            postfix: PostfixRvalue {
                loc: self.loc.clone(),
                expr: PostfixExpression::from(&Primary::Identifier(any_hit_ident.clone())),
            },
        });

        // if !__for__any_hit { <do the else block if any> }
        let if_not_any_hit = IfCondPiece {
            loc: self.loc.clone(),
            expression: Box::new(check_any_hit_expr),
            then: CodeBlock {
                loc: self.loc.clone(),
                entries: if let Some(els) = &self.els {
                    els.then.entries.clone()
                } else {
                    vec![]
                },
            },
        };

        let if_not_any_hit = Statement::IfStatement(IfStatement {
            loc: self.loc.clone(),
            iff: IfPiece {
                content: if_not_any_hit,
            },
            elsif: vec![],
            els: None,
        });

        // __for__any_hit = true;
        let assign_to_any_hit = Statement::AssignStatement(AssignStatement {
            loc: self.loc.clone(),
            id: PostfixExpression::from(&Primary::Identifier(any_hit_ident.clone())),
            val: true_cond.clone(),
        });

        // case Some(x)
        let case_some_blk = MatchRule::enum_and_case(
            self.loc.clone(),
            "Maybe",
            "Some",
            Some(self.id.clone()),
            CodeBlock {
                loc: self.loc.clone(),
                entries: vec![assign_to_any_hit, Statement::CodeBlock(self.then.clone())],
            },
        );

        // case None
        let case_none_block = MatchRule::enum_and_case(
            self.loc.clone(),
            "Maybe",
            "None",
            None,
            CodeBlock {
                loc: self.loc.clone(),
                entries: vec![
                    if_not_any_hit,
                    Statement::BreakStatement(BreakStatement {
                        loc: self.loc.clone(),
                    }),
                ],
            },
        );

        // RuntimeError::UnexpectedType
        let unexpected_type = Expression::from(&PostfixExpression {
            loc: self.loc.clone(),
            base: Primary::Identifier(Identifier {
                loc: self.loc.clone(),
                value: "RuntimeError".to_owned(),
            }),
            terms: vec![PostfixTerm::PostfixTermEnumCase(PostfixTermEnumCase {
                loc: self.loc.clone(),
                id: Identifier {
                    loc: self.loc.clone(),
                    value: "UnexpectedType".to_owned(),
                },
                payload: None,
            })],
        });

        // throw UT
        let throw_ut = Statement::ThrowStatement(ThrowStatement {
            loc: self.loc.clone(),
            val: unexpected_type,
        });

        // read __for__next
        let read_for_next = Expression::from(&PostfixExpression::from(&Primary::Identifier(
            val_next_ident.clone(),
        )));

        // match __for__next { some(x), none, els => throw UT }
        let match_for_next = Statement::MatchStatement(MatchStatement {
            loc: self.loc.clone(),
            expr: read_for_next,
            rules: vec![case_some_blk, case_none_block],
            els: Some(ElsePiece {
                loc: self.loc.clone(),
                then: CodeBlock {
                    loc: self.loc.clone(),
                    entries: vec![throw_ut],
                },
            }),
        });

        // while (true) { fetch_next; match_next; }
        let while_body = CodeBlock {
            loc: self.loc.clone(),
            entries: vec![fetch_next_val, match_for_next],
        };

        // this is while true { do the body }
        let w = Statement::WhileStatement(WhileStatement {
            loc: self.loc.clone(),
            cond: true_cond,
            then: while_body,
            els: None,
        });

        // create the iterator and the any_hit marker, then loop over the iterator
        let blk = CodeBlock {
            loc: self.loc.clone(),
            entries: vec![fetch_iter_val, any_hit_val, w],
        };
        blk.do_compile(params)
    }
}
