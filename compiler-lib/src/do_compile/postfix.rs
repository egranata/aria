// SPDX-License-Identifier: Apache-2.0
use aria_parser::ast::{Expression, ExpressionList, Identifier, SourcePointer};

use crate::{constant_value::ConstantValue, func_builder::BasicBlockOpcode};

use super::{
    CompilationError, CompilationErrorReason, CompilationResult, CompileNode, CompileParams,
};

#[allow(clippy::large_enum_variant)]
pub(super) enum PostfixValue {
    Primary(Box<aria_parser::ast::Primary>),
    Attribute(Box<PostfixValue>, Box<Identifier>),
    Call(Box<PostfixValue>, Box<ExpressionList>, SourcePointer),
    Case(Box<PostfixValue>, Box<Identifier>, Option<Expression>),
    Index(Box<PostfixValue>, Box<aria_parser::ast::Expression>),
    ObjWrite(
        Box<PostfixValue>,
        Vec<(Identifier, aria_parser::ast::Expression)>,
    ),
    ContainerWrite(
        Box<PostfixValue>,
        Vec<(aria_parser::ast::Expression, aria_parser::ast::Expression)>,
    ),
}

impl<'a> PostfixValue {
    pub(super) fn emit_read(&self, params: &'a mut CompileParams) -> CompilationResult {
        match self {
            PostfixValue::Primary(primary) => primary.do_compile(params),
            PostfixValue::Call(base, args, loc) => {
                for expr in args.expressions.iter().rev() {
                    expr.do_compile(params)?;
                }
                let argc = args.expressions.len();
                base.emit_read(params)?;
                params
                    .writer
                    .get_current_block()
                    .write_opcode_and_source_info(BasicBlockOpcode::Call(argc as u8), loc.clone());
                Ok(())
            }
            PostfixValue::Case(base, case, payload) => {
                if let Some(p) = payload {
                    p.do_compile(params)?;
                }
                base.emit_read(params)?;
                let identifier_idx = match params
                    .module
                    .constants
                    .insert(ConstantValue::String(case.value.clone()))
                {
                    Ok(c) => c,
                    Err(_) => {
                        return Err(CompilationError {
                            loc: case.loc.clone(),
                            reason: CompilationErrorReason::TooManyConstants,
                        });
                    }
                };
                params
                    .writer
                    .get_current_block()
                    .write_opcode_and_source_info(
                        BasicBlockOpcode::NewEnumVal(identifier_idx),
                        case.loc.clone(),
                    );
                Ok(())
            }
            PostfixValue::Index(base, index) => {
                base.emit_read(params)?;
                index.do_compile(params)?;
                params
                    .writer
                    .get_current_block()
                    .write_opcode_and_source_info(BasicBlockOpcode::ReadIndex, index.loc().clone());
                Ok(())
            }
            PostfixValue::Attribute(base, identifier) => {
                let identifier_idx = match params
                    .module
                    .constants
                    .insert(ConstantValue::String(identifier.value.clone()))
                {
                    Ok(c) => c,
                    Err(_) => {
                        return Err(CompilationError {
                            loc: identifier.loc.clone(),
                            reason: CompilationErrorReason::TooManyConstants,
                        });
                    }
                };
                base.emit_read(params)?;
                params
                    .writer
                    .get_current_block()
                    .write_opcode_and_source_info(
                        BasicBlockOpcode::ReadAttribute(identifier_idx),
                        identifier.loc.clone(),
                    );
                Ok(())
            }
            PostfixValue::ObjWrite(base, terms) => {
                base.emit_read(params)?;
                for term in terms {
                    params
                        .writer
                        .get_current_block()
                        .write_opcode_and_source_info(BasicBlockOpcode::Dup, term.0.loc.clone());
                    term.1.do_compile(params)?;
                    let identifier_idx = match params
                        .module
                        .constants
                        .insert(ConstantValue::String(term.0.value.clone()))
                    {
                        Ok(c) => c,
                        Err(_) => {
                            return Err(CompilationError {
                                loc: term.0.loc.clone(),
                                reason: CompilationErrorReason::TooManyConstants,
                            });
                        }
                    };
                    params
                        .writer
                        .get_current_block()
                        .write_opcode_and_source_info(
                            BasicBlockOpcode::WriteAttribute(identifier_idx),
                            term.0.loc.clone(),
                        );
                }
                Ok(())
            }
            PostfixValue::ContainerWrite(base, terms) => {
                base.emit_read(params)?;
                for term in terms {
                    params
                        .writer
                        .get_current_block()
                        .write_opcode_and_source_info(BasicBlockOpcode::Dup, term.0.loc().clone());
                    term.0.do_compile(params)?;
                    term.1.do_compile(params)?;
                    params
                        .writer
                        .get_current_block()
                        .write_opcode_and_source_info(
                            BasicBlockOpcode::WriteIndex,
                            term.0.loc().clone(),
                        );
                }
                Ok(())
            }
        }
    }

    pub(super) fn emit_write(
        &self,
        val: &aria_parser::ast::Expression,
        params: &'a mut CompileParams,
    ) -> CompilationResult {
        match self {
            PostfixValue::Primary(primary) => match primary.as_ref() {
                aria_parser::ast::Primary::Identifier(id) => {
                    val.do_compile(params)?;
                    params.scope.emit_write(
                        &id.value,
                        &mut params.module.constants,
                        params.writer.get_current_block(),
                        primary.loc().clone(),
                    )?;
                    Ok(())
                }
                _ => Err(CompilationError {
                    loc: primary.loc().clone(),
                    reason: CompilationErrorReason::ReadOnlyValue,
                }),
            },
            PostfixValue::Call(.., loc) => Err(CompilationError {
                loc: loc.clone(),
                reason: CompilationErrorReason::ReadOnlyValue,
            }),
            PostfixValue::Case(_, case, _) => Err(CompilationError {
                loc: case.loc.clone(),
                reason: CompilationErrorReason::ReadOnlyValue,
            }),
            PostfixValue::Index(base, index) => {
                base.emit_read(params)?;
                index.do_compile(params)?;
                val.do_compile(params)?;
                params
                    .writer
                    .get_current_block()
                    .write_opcode_and_source_info(
                        BasicBlockOpcode::WriteIndex,
                        index.loc().clone(),
                    );
                Ok(())
            }
            PostfixValue::Attribute(base, identifier) => {
                let identifier_idx = match params
                    .module
                    .constants
                    .insert(ConstantValue::String(identifier.value.clone()))
                {
                    Ok(c) => c,
                    Err(_) => {
                        return Err(CompilationError {
                            loc: identifier.loc.clone(),
                            reason: CompilationErrorReason::TooManyConstants,
                        });
                    }
                };
                base.emit_read(params)?;
                val.do_compile(params)?;
                params
                    .writer
                    .get_current_block()
                    .write_opcode_and_source_info(
                        BasicBlockOpcode::WriteAttribute(identifier_idx),
                        identifier.loc.clone(),
                    );
                Ok(())
            }
            PostfixValue::ObjWrite(_, terms) => Err(CompilationError {
                loc: terms[0].0.loc.clone(),
                reason: CompilationErrorReason::WriteOnlyValue,
            }),
            PostfixValue::ContainerWrite(_, terms) => Err(CompilationError {
                loc: terms[0].0.loc().clone(),
                reason: CompilationErrorReason::WriteOnlyValue,
            }),
        }
    }
}

impl From<&aria_parser::ast::PostfixExpression> for PostfixValue {
    fn from(value: &aria_parser::ast::PostfixExpression) -> Self {
        let mut current = PostfixValue::Primary(Box::new(value.base.clone()));
        for term in &value.terms {
            match term {
                aria_parser::ast::PostfixTerm::PostfixTermAttribute(attr) => {
                    current = PostfixValue::Attribute(Box::new(current), Box::new(attr.id.clone()))
                }
                aria_parser::ast::PostfixTerm::PostfixTermIndex(index) => {
                    current = PostfixValue::Index(Box::new(current), Box::new(index.index.clone()))
                }
                aria_parser::ast::PostfixTerm::PostfixTermCall(call) => {
                    current = PostfixValue::Call(
                        Box::new(current),
                        Box::new(call.args.clone()),
                        call.loc.clone(),
                    )
                }
                aria_parser::ast::PostfixTerm::PostfixTermEnumCase(case) => {
                    current = PostfixValue::Case(
                        Box::new(current),
                        Box::new(case.id.clone()),
                        case.payload.clone(),
                    )
                }
                aria_parser::ast::PostfixTerm::PostfixTermObjectWrite(wrt) => {
                    let terms = wrt
                        .terms
                        .terms
                        .iter()
                        .map(|w| (w.id.clone(), w.val.clone()))
                        .collect();
                    current = PostfixValue::ObjWrite(Box::new(current), terms)
                }
                aria_parser::ast::PostfixTerm::PostfixTermContainerWrite(wrt) => {
                    let terms = wrt
                        .terms
                        .terms
                        .iter()
                        .map(|w| (w.idx.clone(), w.val.clone()))
                        .collect();
                    current = PostfixValue::ContainerWrite(Box::new(current), terms)
                }
            }
        }

        current
    }
}
