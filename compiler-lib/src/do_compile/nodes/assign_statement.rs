// SPDX-License-Identifier: Apache-2.0

use aria_parser::ast::{
    DeclarationId, Expression, Identifier, IntLiteral, Primary, Statement, ValDeclEntry,
    ValDeclStatement,
};

use crate::do_compile::{CompilationResult, CompileNode, CompileParams, postfix::PostfixValue};

impl<'a> CompileNode<'a> for aria_parser::ast::AssignStatement {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        if self.id.len() != self.val.len() {
            return Err(crate::do_compile::CompilationError {
                loc: self.loc.clone(),
                reason: crate::do_compile::CompilationErrorReason::AssignmentArityMismatch(
                    self.id.len(),
                    self.val.len(),
                ),
            });
        }
        if self.id.len() == 1 {
            let pv = PostfixValue::from(&self.id[0]);
            pv.emit_write(&self.val[0], params)
        } else {
            let temp_buffer_store = Identifier {
                loc: self.loc.clone(),
                value: format!("__multiwrite_temp_store{:?}", self.loc.location),
            };
            let zero_val = Expression::from(&Primary::IntLiteral(IntLiteral {
                loc: self.loc.clone(),
                val: "0".to_string(),
            }));
            let temp_buffer_expression = Expression::from(&temp_buffer_store.clone());

            Statement::ValDeclStatement(ValDeclStatement {
                loc: self.loc.clone(),
                decls: vec![ValDeclEntry {
                    loc: self.loc.clone(),
                    id: DeclarationId::from(&temp_buffer_store),
                    val: zero_val,
                }],
            })
            .do_compile(params)?;

            for rhs in self.val.iter().rev() {
                rhs.do_compile(params)?;
            }

            for lhs in self.id.iter() {
                params.scope.emit_write(
                    &temp_buffer_store.value,
                    &mut params.module.constants,
                    params.writer.get_current_block(),
                    lhs.loc.clone(),
                )?;
                let pv = PostfixValue::from(lhs);
                pv.emit_write(&temp_buffer_expression, params)?;
            }

            Ok(())
        }
    }
}
