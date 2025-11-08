// SPDX-License-Identifier: Apache-2.0
use aria_parser::ast::{
    AddOperation, AddSymbol, AssignStatement, CompOperation, Expression, LogOperation,
    MulOperation, MulSymbol, ParenExpression, PostfixExpression, PostfixRvalue, Primary,
    RelOperation, ShiftOperation, ShiftSymbol, UnaryOperation,
};

use crate::do_compile::{CompilationResult, CompileNode, CompileParams};

impl<'a> CompileNode<'a> for aria_parser::ast::WriteOpEqStatement {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        let rhs_as_unary = UnaryOperation::from(&PostfixRvalue::from(&PostfixExpression::from(
            &Primary::ParenExpression(ParenExpression {
                loc: self.val.loc().clone(),
                value: Box::new(self.val.clone()),
            }),
        )));

        let rhs_as_mul = MulOperation::from(&rhs_as_unary);

        let add_op = match self.op {
            aria_parser::ast::AddEqSymbol::PlusEq => AddOperation {
                loc: self.loc.clone(),
                left: MulOperation::from(&UnaryOperation::from(&PostfixRvalue::from(&self.id))),
                right: vec![(AddSymbol::Plus, rhs_as_mul)],
            },
            aria_parser::ast::AddEqSymbol::MinusEq => AddOperation {
                loc: self.loc.clone(),
                left: MulOperation::from(&UnaryOperation::from(&PostfixRvalue::from(&self.id))),
                right: vec![(AddSymbol::Minus, rhs_as_mul)],
            },
            aria_parser::ast::AddEqSymbol::StarEq => {
                let mo = MulOperation {
                    loc: self.loc.clone(),
                    left: UnaryOperation::from(&PostfixRvalue::from(&self.id)),
                    right: vec![(MulSymbol::Star, rhs_as_unary)],
                };
                AddOperation {
                    loc: self.loc.clone(),
                    left: mo,
                    right: vec![],
                }
            }
            aria_parser::ast::AddEqSymbol::SlashEq => {
                let mo = MulOperation {
                    loc: self.loc.clone(),
                    left: UnaryOperation::from(&PostfixRvalue::from(&self.id)),
                    right: vec![(MulSymbol::Slash, rhs_as_unary)],
                };
                AddOperation {
                    loc: self.loc.clone(),
                    left: mo,
                    right: vec![],
                }
            }
            aria_parser::ast::AddEqSymbol::PercentEq => {
                let mo = MulOperation {
                    loc: self.loc.clone(),
                    left: UnaryOperation::from(&PostfixRvalue::from(&self.id)),
                    right: vec![(MulSymbol::Percent, rhs_as_unary)],
                };
                AddOperation {
                    loc: self.loc.clone(),
                    left: mo,
                    right: vec![],
                }
            }
            aria_parser::ast::AddEqSymbol::ShiftLeftEq => {
                ShiftOperation {
                    loc: self.loc.clone(),
                    left: AddOperation::from(&MulOperation::from(&UnaryOperation::from(
                        &PostfixRvalue::from(&self.id),
                    ))),
                    right: Some((ShiftSymbol::Leftward, AddOperation::from(&rhs_as_mul))),
                };
                let mo = MulOperation {
                    loc: self.loc.clone(),
                    left: UnaryOperation::from(&PostfixRvalue::from(&self.id)),
                    right: vec![],
                };
                AddOperation {
                    loc: self.loc.clone(),
                    left: mo,
                    right: vec![],
                }
            }
            aria_parser::ast::AddEqSymbol::ShiftRightEq => {
                ShiftOperation {
                    loc: self.loc.clone(),
                    left: AddOperation::from(&MulOperation::from(&UnaryOperation::from(
                        &PostfixRvalue::from(&self.id),
                    ))),
                    right: Some((ShiftSymbol::Leftward, AddOperation::from(&rhs_as_mul))),
                };
                let mo = MulOperation {
                    loc: self.loc.clone(),
                    left: UnaryOperation::from(&PostfixRvalue::from(&self.id)),
                    right: vec![],
                };
                AddOperation {
                    loc: self.loc.clone(),
                    left: mo,
                    right: vec![],
                }
            }
            aria_parser::ast::AddEqSymbol::XorEq
            | aria_parser::ast::AddEqSymbol::AndEq
            | aria_parser::ast::AddEqSymbol::OrEq => todo!(),
        };

        let assign_stmt = AssignStatement {
            loc: self.loc.clone(),
            id: vec![self.id.clone()],
            val: vec![Expression::from(&LogOperation::from(&CompOperation::from(
                &RelOperation::from(&ShiftOperation::from(&add_op)),
            )))],
        };

        assign_stmt.do_compile(params)
    }
}
