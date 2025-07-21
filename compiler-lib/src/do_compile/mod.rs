// SPDX-License-Identifier: Apache-2.0
use std::{collections::HashSet, rc::Rc};

use aria_parser::ast::{
    source_to_ast, ArgumentList, AssertStatement, CodeBlock, DeclarationId, ElsePiece,
    EnumCaseDecl, EnumDecl, EnumDeclEntry, Expression, Identifier, MatchPattern,
    MatchPatternEnumCase, MatchRule, MatchStatement, MethodAccess, MethodDecl, MixinIncludeDecl,
    ParsedModule, ReturnStatement, SourceBuffer, SourcePointer, Statement, StringLiteral,
    StructDecl, StructEntry, ValDeclStatement,
};
use haxby_opcodes::{builtin_type_ids::BUILTIN_TYPE_ANY, function_attribs::*};
use thiserror::Error;

use crate::{
    constant_value::{CompiledCodeObject, ConstantValue, ConstantValuesError},
    func_builder::{BasicBlock, BasicBlockOpcode, FunctionBuilder},
    module::CompiledModule,
    scope::{CompilationScope, ScopeError, ScopeErrorReason},
    CompilationOptions,
};

#[derive(Debug, Error)]
pub enum CompilationErrorReason {
    #[error("identifier is reserved: '{0}'")]
    ReservedIdentifier(String),
    #[error("no such identifier: '{0}'")]
    NoSuchIdentifier(String),
    #[error("function body is larger than allowed")]
    FunctionBodyTooLarge,
    #[error("list length is out of bounds")]
    ListTooLarge,
    #[error("attempt to modify a read-only value")]
    ReadOnlyValue,
    #[error("{0} is not a valid literal")]
    InvalidLiteral(String),
    #[error("attempt to read a write-only value")]
    WriteOnlyValue,
    #[error("parser error: {0}")]
    ParserError(String),
    #[error("module contains too many constant values")]
    TooManyConstants,
    #[error("function accepts too many arguments")]
    TooManyArguments,
    #[error("flow control statement not permitted in current context")]
    FlowControlNotAllowed,
    #[error("argument name '{0}' is already defined for this function")]
    DuplicateArgumentName(String),
    #[error("struct members do not support type hints")]
    NoTypeHintOnStructMember,
    #[error("nested closures are not supported")]
    NestedClosureDisallowed,
}

impl From<&ScopeErrorReason> for CompilationErrorReason {
    fn from(value: &ScopeErrorReason) -> Self {
        match value {
            ScopeErrorReason::TooManyConstants => CompilationErrorReason::TooManyConstants,
            ScopeErrorReason::NoSuchIdentifier(s) => {
                CompilationErrorReason::NoSuchIdentifier(s.clone())
            }
            ScopeErrorReason::OverlyDeepClosure => CompilationErrorReason::NestedClosureDisallowed,
        }
    }
}

impl From<ScopeError> for CompilationError {
    fn from(value: ScopeError) -> Self {
        CompilationError {
            loc: value.loc,
            reason: CompilationErrorReason::from(&value.reason),
        }
    }
}

impl From<&ConstantValuesError> for CompilationErrorReason {
    fn from(value: &ConstantValuesError) -> Self {
        match value {
            ConstantValuesError::OutOfSpace => CompilationErrorReason::TooManyConstants,
        }
    }
}

pub struct CompilationError {
    pub loc: SourcePointer,
    pub reason: CompilationErrorReason,
}

impl std::fmt::Display for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "at {}, error occurred: {}", self.loc, self.reason)
    }
}

impl std::fmt::Debug for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "at {}, error occurred: {}", self.loc, self.reason)
    }
}

pub type CompilationResult<T = (), E = CompilationError> = Result<T, E>;

#[derive(Default)]
struct ControlFlowTargets {
    break_dest: Option<Rc<BasicBlock>>,
    continue_dest: Option<Rc<BasicBlock>>,
}

struct CompileParams<'a> {
    module: &'a mut CompiledModule,
    scope: &'a CompilationScope,
    writer: &'a mut FunctionBuilder,
    cflow: &'a ControlFlowTargets,
    options: &'a CompilationOptions,
}

trait CompileNode<'a, T = (), E = CompilationError> {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult<T, E>;

    fn insert_const_or_fail(
        &self,
        params: &'a mut CompileParams,
        ct: ConstantValue,
        loc: &SourcePointer,
    ) -> CompilationResult<u16> {
        match params.module.constants.insert(ct) {
            Ok(idx) => Ok(idx),
            Err(_) => Err(CompilationError {
                loc: loc.clone(),
                reason: CompilationErrorReason::TooManyConstants,
            }),
        }
    }
}

mod nodes;
mod postfix;

fn ensure_unique_arg_names(args: &ArgumentList) -> CompilationResult {
    let mut arg_set = HashSet::new();
    for arg in &args.names {
        if arg_set.contains(&arg.name.value) {
            return Err(CompilationError {
                loc: arg.loc.clone(),
                reason: CompilationErrorReason::DuplicateArgumentName(arg.name.value.clone()),
            });
        } else {
            arg_set.insert(arg.name.value.clone());
        }
    }

    Ok(())
}

fn emit_args_at_target(args: &ArgumentList, params: &mut CompileParams) -> CompilationResult {
    for arg in &args.names {
        if let Some(ty) = &arg.ty {
            ty.do_compile(params)?;
        } else {
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(
                    BasicBlockOpcode::PushBuiltinTy(BUILTIN_TYPE_ANY),
                    arg.loc.clone(),
                );
        }
        params.scope.emit_typed_define(
            &arg.name.value,
            &mut params.module.constants,
            params.writer.get_current_block(),
            arg.loc.clone(),
        )?;
        params.scope.emit_write(
            &arg.name.value,
            &mut params.module.constants,
            params.writer.get_current_block(),
            arg.loc.clone(),
        )?;
    }
    Ok(())
}

fn compile_method_decl(pf: &MethodDecl, params: &mut CompileParams) -> CompilationResult {
    if pf.args.names.len() > u8::MAX.into() {
        return Err(CompilationError {
            loc: pf.loc.clone(),
            reason: CompilationErrorReason::TooManyArguments,
        });
    }

    let scope = CompilationScope::function(params.scope);
    let mut writer = FunctionBuilder::default();
    let cflow = ControlFlowTargets::default();
    let mut c_params = CompileParams {
        module: params.module,
        scope: &scope,
        writer: &mut writer,
        cflow: &cflow,
        options: params.options,
    };
    let arity: u8 = pf.args.names.len() as u8 + 1;
    pf.do_compile(&mut c_params)?;
    let co = match writer.write(&params.module.constants, params.options) {
        Ok(c) => c,
        Err(er) => {
            return Err(CompilationError {
                loc: pf.loc.clone(),
                reason: er,
            })
        }
    };
    let frame_size = scope.as_function_root().unwrap().num_locals();
    let line_table = writer.write_line_table().clone();
    let cco = CompiledCodeObject {
        name: pf.name.value.clone(),
        body: co,
        arity,
        loc: pf.loc.clone(),
        line_table,
        frame_size,
    };
    let cco_idx =
        pf.insert_const_or_fail(params, ConstantValue::CompiledCodeObject(cco), &pf.loc)?;
    params
        .writer
        .get_current_block()
        .write_opcode_and_source_info(BasicBlockOpcode::Push(cco_idx), pf.loc.clone());
    Ok(())
}

// assume your parent struct is on the stack
fn emit_type_mixin_include_decl_compile(
    mi: &MixinIncludeDecl,
    params: &mut CompileParams,
) -> CompilationResult {
    mi.what.do_compile(params)?;
    params
        .writer
        .get_current_block()
        .write_opcode_and_source_info(BasicBlockOpcode::IncludeMixin, mi.loc.clone());
    Ok(())
}

// assume your parent struct is on the stack
fn emit_method_decl_compile(md: &MethodDecl, params: &mut CompileParams) -> CompilationResult {
    compile_method_decl(md, params)?;
    let name_idx = md.insert_const_or_fail(
        params,
        ConstantValue::String(md.name.value.clone()),
        &md.loc,
    )?;
    params
        .writer
        .get_current_block()
        .write_opcode_and_source_info(
            BasicBlockOpcode::BindMethod(
                if md.vararg { FUNC_ACCEPTS_VARARG } else { 0 }
                    | FUNC_IS_METHOD
                    | if md.access == MethodAccess::Type {
                        METHOD_ATTRIBUTE_TYPE
                    } else {
                        0
                    },
                name_idx,
            ),
            md.loc.clone(),
        );

    Ok(())
}

// assume your parent struct is on the stack
fn emit_type_val_decl_compile(
    vd: &ValDeclStatement,
    params: &mut CompileParams,
) -> CompilationResult {
    if vd.id.ty.is_some() {
        return Err(CompilationError {
            loc: vd.loc.clone(),
            reason: CompilationErrorReason::NoTypeHintOnStructMember,
        });
    }
    vd.val.do_compile(params)?;
    let name_idx = vd.insert_const_or_fail(
        params,
        ConstantValue::String(vd.id.name.value.clone()),
        &vd.loc,
    )?;
    params
        .writer
        .get_current_block()
        .write_opcode_and_source_info(BasicBlockOpcode::WriteAttribute(name_idx), vd.loc.clone());
    Ok(())
}

// assume your parent struct is on the stack
fn emit_type_members_compile(
    entries: &[StructEntry],
    params: &mut CompileParams,
    drop_at_end: bool,
) -> CompilationResult {
    for se in entries {
        params
            .writer
            .get_current_block()
            .write_opcode_and_source_info(BasicBlockOpcode::Dup, se.loc().clone());

        match se {
            aria_parser::ast::StructEntry::Method(md) => emit_method_decl_compile(md, params)?,
            aria_parser::ast::StructEntry::Variable(vd) => emit_type_val_decl_compile(vd, params)?,
            aria_parser::ast::StructEntry::Struct(sd) => {
                do_struct_compile(sd, params)?;

                let name_idx = sd.insert_const_or_fail(
                    params,
                    ConstantValue::String(sd.name.value.clone()),
                    &sd.loc,
                )?;
                params
                    .writer
                    .get_current_block()
                    .write_opcode_and_source_info(
                        BasicBlockOpcode::WriteAttribute(name_idx),
                        sd.loc.clone(),
                    );
            }
            aria_parser::ast::StructEntry::Enum(ed) => {
                do_enum_compile(ed, params, |name, params| {
                    params
                        .writer
                        .get_current_block()
                        .write_opcode_and_source_info(BasicBlockOpcode::Swap, ed.loc.clone());
                    params
                        .writer
                        .get_current_block()
                        .write_opcode_and_source_info(BasicBlockOpcode::Copy(1), ed.loc.clone());

                    let name_idx = ed.insert_const_or_fail(
                        params,
                        ConstantValue::String(name.to_owned()),
                        &ed.loc,
                    )?;

                    params
                        .writer
                        .get_current_block()
                        .write_opcode_and_source_info(
                            BasicBlockOpcode::WriteAttribute(name_idx),
                            ed.loc.clone(),
                        );
                    Ok(())
                })?;
            }
            aria_parser::ast::StructEntry::MixinInclude(mi) => {
                emit_type_mixin_include_decl_compile(mi, params)?
            }
        }
    }

    if drop_at_end {
        // remove the last leftover struct
        #[allow(deprecated)] // no entry to ascribe this write to
        params
            .writer
            .get_current_block()
            .write_opcode(BasicBlockOpcode::Pop);
    }

    Ok(())
}

fn do_struct_compile(sd: &StructDecl, params: &mut CompileParams) -> CompilationResult {
    let self_name = StringLiteral {
        loc: sd.loc.clone(),
        value: sd.name.value.clone(),
    };
    self_name.do_compile(params)?;
    params
        .writer
        .get_current_block()
        .write_opcode_and_source_info(BasicBlockOpcode::BuildStruct, sd.loc.clone());

    emit_type_members_compile(&sd.body, params, false)
}

fn do_enum_compile<T>(
    ed: &EnumDecl,
    params: &mut CompileParams,
    name_writer: T,
) -> CompilationResult
where
    T: FnOnce(&str, &mut CompileParams) -> CompilationResult,
{
    let self_name = StringLiteral {
        loc: ed.loc.clone(),
        value: ed.name.value.clone(),
    };
    self_name.do_compile(params)?;

    params
        .writer
        .get_current_block()
        .write_opcode_and_source_info(BasicBlockOpcode::BuildEnum, ed.loc.clone());
    name_writer(&ed.name.value, params)?;

    let mut cases: Vec<EnumCaseDecl> = vec![];
    let mut entries: Vec<StructEntry> = vec![];

    for ede in &ed.body {
        match ede {
            EnumDeclEntry::EnumCaseDecl(case) => cases.push(case.clone()),
            EnumDeclEntry::StructEntry(entry) => entries.push(entry.clone()),
        }
    }

    emit_type_members_compile(&entries, params, false)?;

    emit_enum_cases(&cases, params)?;

    let enum_helper_methods = generate_case_helpers_extension_for_enum(&cases);
    emit_type_members_compile(&enum_helper_methods, params, true)
}

fn generate_is_case_helper_for_enum(case: &EnumCaseDecl) -> MethodDecl {
    let return_true_stmt = ReturnStatement::from(&Expression::from(&Identifier {
        loc: case.loc.clone(),
        value: "true".to_owned(),
    }));

    let return_false_stmt = ReturnStatement::from(&Expression::from(&Identifier {
        loc: case.loc.clone(),
        value: "false".to_owned(),
    }));

    let match_case_pattern = MatchPatternEnumCase {
        loc: case.loc.clone(),
        case: case.name.clone(),
        payload: None,
    };

    let match_rule = MatchRule {
        loc: case.loc.clone(),
        patterns: vec![MatchPattern::MatchPatternEnumCase(match_case_pattern)],
        then: CodeBlock::from(&Statement::ReturnStatement(return_true_stmt)),
    };

    let match_statement = MatchStatement {
        loc: case.loc.clone(),
        expr: Expression::from(&Identifier {
            loc: case.loc.clone(),
            value: "this".to_owned(),
        }),
        rules: vec![match_rule],
        els: Some(ElsePiece {
            loc: case.loc.clone(),
            then: CodeBlock::from(&Statement::ReturnStatement(return_false_stmt)),
        }),
    };

    let method_body = CodeBlock::from(&Statement::MatchStatement(match_statement));
    MethodDecl {
        loc: case.loc.clone(),
        access: MethodAccess::Instance,
        name: Identifier {
            loc: case.loc.clone(),
            value: format!("is_{}", case.name.value),
        },
        args: ArgumentList::empty(case.loc.clone()),
        vararg: false,
        body: method_body,
    }
}

fn generate_unwap_case_helper_for_enum(case: &EnumCaseDecl) -> MethodDecl {
    assert!(case.payload.is_some());

    let return_true_stmt = ReturnStatement::from(&Expression::from(&Identifier {
        loc: case.loc.clone(),
        value: "__case_payload".to_owned(),
    }));

    let assert_false_stmt = AssertStatement {
        loc: case.loc.clone(),
        val: Expression::from(&Identifier {
            loc: case.loc.clone(),
            value: "false".to_owned(),
        }),
    };

    let match_case_pattern = MatchPatternEnumCase {
        loc: case.loc.clone(),
        case: case.name.clone(),
        payload: Some(DeclarationId {
            loc: case.loc.clone(),
            name: Identifier {
                loc: case.loc.clone(),
                value: "__case_payload".to_owned(),
            },
            ty: None,
        }),
    };

    let match_rule = MatchRule {
        loc: case.loc.clone(),
        patterns: vec![MatchPattern::MatchPatternEnumCase(match_case_pattern)],
        then: CodeBlock::from(&Statement::ReturnStatement(return_true_stmt)),
    };

    let match_statement = MatchStatement {
        loc: case.loc.clone(),
        expr: Expression::from(&Identifier {
            loc: case.loc.clone(),
            value: "this".to_owned(),
        }),
        rules: vec![match_rule],
        els: Some(ElsePiece {
            loc: case.loc.clone(),
            then: CodeBlock::from(&Statement::AssertStatement(assert_false_stmt)),
        }),
    };

    let method_body = CodeBlock::from(&Statement::MatchStatement(match_statement));
    MethodDecl {
        loc: case.loc.clone(),
        access: MethodAccess::Instance,
        name: Identifier {
            loc: case.loc.clone(),
            value: format!("unwrap_{}", case.name.value),
        },
        args: ArgumentList::empty(case.loc.clone()),
        vararg: false,
        body: method_body,
    }
}

fn generate_case_helpers_extension_for_enum(cases: &[EnumCaseDecl]) -> Vec<StructEntry> {
    let mut entries: Vec<StructEntry> = vec![];

    for case in cases {
        entries.push(StructEntry::Method(Box::new(
            generate_is_case_helper_for_enum(case),
        )));
        if case.payload.is_some() {
            entries.push(StructEntry::Method(Box::new(
                generate_unwap_case_helper_for_enum(case),
            )));
        }
    }

    entries
}

// assume your parent enum is on the stack
fn emit_enum_cases(cases: &[EnumCaseDecl], params: &mut CompileParams) -> CompilationResult {
    for case in cases {
        case.do_compile(params)?;
    }

    Ok(())
}

pub(crate) fn compile_from_ast(
    ast: &ParsedModule,
    options: &CompilationOptions,
) -> CompilationResult<CompiledModule, Vec<CompilationError>> {
    let mut dest = CompiledModule::default();
    let scope = CompilationScope::module();
    let mut mod_init_bytecode = FunctionBuilder::default();
    let cflow = ControlFlowTargets::default();

    let mut c_params = CompileParams {
        module: &mut dest,
        scope: &scope,
        writer: &mut mod_init_bytecode,
        cflow: &cflow,
        options,
    };

    ast.do_compile(&mut c_params)?;

    Ok(dest)
}

pub(crate) fn compile_from_source(
    src: &SourceBuffer,
    options: &CompilationOptions,
) -> CompilationResult<CompiledModule, Vec<CompilationError>> {
    let ast = match source_to_ast(src) {
        Ok(ast) => ast,
        Err(err) => {
            return Err(vec![CompilationError {
                loc: err.loc,
                reason: CompilationErrorReason::ParserError(err.msg),
            }]);
        }
    };

    compile_from_ast(&ast, options)
}
