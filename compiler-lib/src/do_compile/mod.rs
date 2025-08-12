// SPDX-License-Identifier: Apache-2.0
use std::{collections::HashSet, rc::Rc};

use aria_parser::ast::{
    ArgumentList, AssertStatement, CodeBlock, DeclarationId, ElsePiece, EnumCaseDecl, EnumDecl,
    EnumDeclEntry, Expression, Identifier, MatchPattern, MatchPatternEnumCase, MatchRule,
    MatchStatement, MethodAccess, MethodDecl, MixinIncludeDecl, OperatorDecl, ParsedModule,
    ReturnStatement, SourceBuffer, SourcePointer, Statement, StringLiteral, StructDecl,
    StructEntry, ValDeclStatement, prettyprint::PrettyPrintable, source_to_ast,
};
use haxby_opcodes::{builtin_type_ids::BUILTIN_TYPE_ANY, function_attribs::*};
use thiserror::Error;

use crate::{
    CompilationOptions,
    constant_value::{ConstantValue, ConstantValuesError},
    func_builder::{BasicBlock, BasicBlockOpcode, FunctionBuilder},
    module::CompiledModule,
    scope::{CompilationScope, ScopeError, ScopeErrorReason},
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
    #[error("{0} is not a valid operator")]
    InvalidOperator(String),
    #[error("{0} cannot be reversed")]
    IrreversibleOperator(String),
    #[error("operator {0} accepts {1} arguments, but {2} were declared")]
    OperatorArityMismatch(String, usize, usize),
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

fn emit_arg_at_target(arg: &DeclarationId, params: &mut CompileParams) -> CompilationResult {
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

    Ok(())
}

#[allow(dead_code)]
struct ArgumentCountInfo {
    user_args: u8,
    total_args: u8,
    varargs: bool,
}

fn emit_args_at_target(
    prefix_args: &[DeclarationId],
    args: &ArgumentList,
    suffix_args: &[DeclarationId],
    params: &mut CompileParams,
) -> CompilationResult<ArgumentCountInfo> {
    ensure_unique_arg_names(args)?;

    let total_args = prefix_args.len() + args.names.len() + suffix_args.len();
    if total_args > u8::MAX.into() {
        return Err(CompilationError {
            loc: args.loc.clone(),
            reason: CompilationErrorReason::TooManyArguments,
        });
    }

    let argc_info = ArgumentCountInfo {
        user_args: args.len() as u8,
        total_args: total_args as u8,
        varargs: args.vararg,
    };

    for arg in prefix_args {
        emit_arg_at_target(arg, params)?;
    }

    for arg in &args.names {
        emit_arg_at_target(arg, params)?;
    }

    for arg in suffix_args {
        emit_arg_at_target(arg, params)?;
    }

    if args.vararg {
        params.scope.emit_untyped_define(
            "varargs",
            &mut params.module.constants,
            params.writer.get_current_block(),
            args.loc.clone(),
        )?;
    }
    Ok(argc_info)
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
    md.do_compile(params)?;

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
                if md.args.vararg {
                    FUNC_ACCEPTS_VARARG
                } else {
                    0
                } | FUNC_IS_METHOD
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

struct OperatorInfo {
    arity: Option<usize>, // number of arguments (minus the receiver this)
    direct_name: &'static str,
    reverse_name: &'static str,
}

lazy_static::lazy_static! {
    static ref OPERATOR_INFO: std::collections::HashMap<&'static str, OperatorInfo> = {
        let mut map = std::collections::HashMap::new();

        map.insert(
            "+",
            OperatorInfo {
                arity: Some(1),
                direct_name: "add",
                reverse_name: "radd",
            },
        );

        map.insert(
            "-",
            OperatorInfo {
                arity: Some(1),
                direct_name: "sub",
                reverse_name: "rsub",
            },
        );

        map.insert(
            "*",
            OperatorInfo {
                arity: Some(1),
                direct_name: "mul",
                reverse_name: "rmul",
            },
        );

        map.insert(
            "/",
            OperatorInfo {
                arity: Some(1),
                direct_name: "div",
                reverse_name: "rdiv",
            },
        );

        map.insert(
            "%",
            OperatorInfo {
                arity: Some(1),
                direct_name: "rem",
                reverse_name: "rrem",
            },
        );

        map.insert(
            "<<",
            OperatorInfo {
                arity: Some(1),
                direct_name: "lshift",
                reverse_name: "rlshift",
            },
        );

        map.insert(
            ">>",
            OperatorInfo {
                arity: Some(1),
                direct_name: "rshift",
                reverse_name: "rrshift",
            },
        );

        map.insert("==",
            OperatorInfo {
                arity: Some(1),
                direct_name: "equals",
                reverse_name: "",
            },
        );

        map.insert("<",
            OperatorInfo {
                arity: Some(1),
                direct_name: "lt",
                reverse_name: "gt",
            },
        );

        map.insert(">",
            OperatorInfo {
                arity: Some(1),
                direct_name: "gt",
                reverse_name: "lt",
            },
        );

        map.insert("<=",
            OperatorInfo {
                arity: Some(1),
                direct_name: "lteq",
                reverse_name: "gteq",
            },
        );

        map.insert(">=",
            OperatorInfo {
                arity: Some(1),
                direct_name: "gteq",
                reverse_name: "lteq",
            },
        );

        map.insert("&",
            OperatorInfo {
                arity: Some(1),
                direct_name: "bwand",
                reverse_name: "rbwand",
            },
        );

        map.insert("|",
            OperatorInfo {
                arity: Some(1),
                direct_name: "bwor",
                reverse_name: "rbwor",
            },
        );

        map.insert("^",
            OperatorInfo {
                arity: Some(1),
                direct_name: "xor",
                reverse_name: "rxor",
            },
        );

        map.insert("u-",
            OperatorInfo {
                arity: Some(0),
                direct_name: "neg",
                reverse_name: "",
            },
        );

        map.insert("()",
            OperatorInfo {
                arity: None, // call operator has no arity, it can take any number of arguments
                direct_name: "call",
                reverse_name: "",
            },
        );

        map.insert("[]",
            OperatorInfo {
                arity: Some(1),
                direct_name: "read_index",
                reverse_name: "",
            },
        );

        map.insert("[]=",
            OperatorInfo {
                arity: Some(2),
                direct_name: "write_index",
                reverse_name: "",
            },
        );

        map
    };
}

// assume your parent struct is on the stack
fn emit_operator_decl_compile(op: &OperatorDecl, params: &mut CompileParams) -> CompilationResult {
    let op_symbol = op
        .symbol
        .prettyprint(
            aria_parser::ast::prettyprint::printout_accumulator::PrintoutAccumulator::default(),
        )
        .value();

    let op_info = match OPERATOR_INFO.get(op_symbol.as_str()) {
        Some(info) => info,
        None => {
            return Err(CompilationError {
                loc: op.loc.clone(),
                reason: CompilationErrorReason::InvalidOperator(op_symbol),
            });
        }
    };

    if let Some(arity) = op_info.arity
        && op.args.len() != arity
    {
        return Err(CompilationError {
            loc: op.loc.clone(),
            reason: CompilationErrorReason::OperatorArityMismatch(op_symbol, arity, op.args.len()),
        });
    }

    if op.reverse && op_info.reverse_name.is_empty() {
        return Err(CompilationError {
            loc: op.loc.clone(),
            reason: CompilationErrorReason::IrreversibleOperator(op_symbol),
        });
    }

    let op_fn_name = format!(
        "_op_impl_{}",
        if op.reverse {
            op_info.reverse_name
        } else {
            op_info.direct_name
        }
    );

    let md = MethodDecl {
        loc: op.loc.clone(),
        access: MethodAccess::Instance,
        name: Identifier {
            loc: op.loc.clone(),
            value: op_fn_name,
        },
        args: op.args.clone(),
        body: op.body.clone(),
    };

    emit_method_decl_compile(&md, params)
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
            aria_parser::ast::StructEntry::Operator(od) => emit_operator_decl_compile(od, params)?,
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

    let enum_helper_methods = generate_case_helpers_extension_for_enum(&cases);
    emit_type_members_compile(&enum_helper_methods, params, false)?;

    emit_type_members_compile(&entries, params, false)?;

    emit_enum_cases(&cases, params)?;

    params
        .writer
        .get_current_block()
        .write_opcode_and_source_info(BasicBlockOpcode::Pop, ed.loc.clone());

    Ok(())
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
