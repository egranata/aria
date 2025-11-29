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

        // We need to handle different operation levels
        // For Add/Mul operations, we build an AddOperation
        // For Shift operations, we build a ShiftOperation
        // For bitwise operations, we build the appropriate operation type

        let final_expr = match self.op {
            // Addition and subtraction - these are AddOperations
            aria_parser::ast::AddEqSymbol::PlusEq => {
                let add_op = AddOperation {
                    loc: self.loc.clone(),
                    left: MulOperation::from(&UnaryOperation::from(&PostfixRvalue::from(&self.id))),
                    right: vec![(AddSymbol::Plus, rhs_as_mul)],
                };
                Expression::from(&LogOperation::from(&CompOperation::from(
                    &RelOperation::from(&ShiftOperation::from(&add_op)),
                )))
            }
            aria_parser::ast::AddEqSymbol::MinusEq => {
                let add_op = AddOperation {
                    loc: self.loc.clone(),
                    left: MulOperation::from(&UnaryOperation::from(&PostfixRvalue::from(&self.id))),
                    right: vec![(AddSymbol::Minus, rhs_as_mul)],
                };
                Expression::from(&LogOperation::from(&CompOperation::from(
                    &RelOperation::from(&ShiftOperation::from(&add_op)),
                )))
            }

            // Multiplication operations - these are MulOperations wrapped in AddOperation
            aria_parser::ast::AddEqSymbol::StarEq => {
                let mo = MulOperation {
                    loc: self.loc.clone(),
                    left: UnaryOperation::from(&PostfixRvalue::from(&self.id)),
                    right: vec![(MulSymbol::Star, rhs_as_unary)],
                };
                let add_op = AddOperation {
                    loc: self.loc.clone(),
                    left: mo,
                    right: vec![],
                };
                Expression::from(&LogOperation::from(&CompOperation::from(
                    &RelOperation::from(&ShiftOperation::from(&add_op)),
                )))
            }
            aria_parser::ast::AddEqSymbol::SlashEq => {
                let mo = MulOperation {
                    loc: self.loc.clone(),
                    left: UnaryOperation::from(&PostfixRvalue::from(&self.id)),
                    right: vec![(MulSymbol::Slash, rhs_as_unary)],
                };
                let add_op = AddOperation {
                    loc: self.loc.clone(),
                    left: mo,
                    right: vec![],
                };
                Expression::from(&LogOperation::from(&CompOperation::from(
                    &RelOperation::from(&ShiftOperation::from(&add_op)),
                )))
            }
            aria_parser::ast::AddEqSymbol::PercentEq => {
                let mo = MulOperation {
                    loc: self.loc.clone(),
                    left: UnaryOperation::from(&PostfixRvalue::from(&self.id)),
                    right: vec![(MulSymbol::Percent, rhs_as_unary)],
                };
                let add_op = AddOperation {
                    loc: self.loc.clone(),
                    left: mo,
                    right: vec![],
                };
                Expression::from(&LogOperation::from(&CompOperation::from(
                    &RelOperation::from(&ShiftOperation::from(&add_op)),
                )))
            }

            // Shift operations - these are ShiftOperations (lower precedence than Add)
            aria_parser::ast::AddEqSymbol::ShiftLeftEq => {
                let shift_op = ShiftOperation {
                    loc: self.loc.clone(),
                    left: AddOperation::from(&MulOperation::from(&UnaryOperation::from(
                        &PostfixRvalue::from(&self.id),
                    ))),
                    right: Some((
                        ShiftSymbol::Leftward,
                        AddOperation::from(&MulOperation::from(&rhs_as_unary)),
                    )),
                };
                Expression::from(&LogOperation::from(&CompOperation::from(
                    &RelOperation::from(&shift_op),
                )))
            }
            aria_parser::ast::AddEqSymbol::ShiftRightEq => {
                let shift_op = ShiftOperation {
                    loc: self.loc.clone(),
                    left: AddOperation::from(&MulOperation::from(&UnaryOperation::from(
                        &PostfixRvalue::from(&self.id),
                    ))),
                    right: Some((
                        ShiftSymbol::Rightward, // Note: was Leftward in your code!
                        AddOperation::from(&MulOperation::from(&rhs_as_unary)),
                    )),
                };
                Expression::from(&LogOperation::from(&CompOperation::from(
                    &RelOperation::from(&shift_op),
                )))
            }

            // Bitwise operations - these need to be at the appropriate level
            // Based on the hierarchy, XOR, AND, OR are likely at CompOperation or LogOperation level
            // You'll need to check your grammar to see where these fit
            aria_parser::ast::AddEqSymbol::XorEq
            | aria_parser::ast::AddEqSymbol::AndEq
            | aria_parser::ast::AddEqSymbol::OrEq => {
                // TODO: Implement these based on where they sit in your operator precedence
                // Are these bitwise ops (&, |, ^) or logical ops (&&, ||)?
                // Check your CompOperation and LogOperation definitions
                todo!("Implement bitwise operations - check operator precedence in your grammar")
            }
        };

        let assign_stmt = AssignStatement {
            loc: self.loc.clone(),
            id: vec![self.id.clone()],
            val: vec![final_expr],
        };

        assign_stmt.do_compile(params)
    }
}
