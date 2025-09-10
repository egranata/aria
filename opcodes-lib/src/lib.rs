// SPDX-License-Identifier: Apache-2.0
pub const OPCODE_NOP: u8 = 0;
pub const OPCODE_PUSH: u8 = 1;
pub const OPCODE_PUSH_0: u8 = 2;
pub const OPCODE_PUSH_1: u8 = 3;
pub const OPCODE_PUSH_TRUE: u8 = 4;
pub const OPCODE_PUSH_FALSE: u8 = 5;
pub const OPCODE_PUSH_BUILTIN_TYPE: u8 = 6;
pub const OPCODE_PUSH_RUNTIME_VALUE: u8 = 7;
pub const OPCODE_POP: u8 = 8;
pub const OPCODE_DUP: u8 = 9;
pub const OPCODE_SWAP: u8 = 10;
pub const OPCODE_COPY: u8 = 11;
// ..
pub const OPCODE_ADD: u8 = 20;
pub const OPCODE_SUB: u8 = 21;
pub const OPCODE_MUL: u8 = 22;
pub const OPCODE_DIV: u8 = 23;
pub const OPCODE_REM: u8 = 24;
pub const OPCODE_NEG: u8 = 25;
pub const OPCODE_SHL: u8 = 26;
pub const OPCODE_SHR: u8 = 27;
// ...
pub const OPCODE_READ_LOCAL: u8 = 30;
pub const OPCODE_WRITE_LOCAL: u8 = 31;
pub const OPCODE_TYPEDEF_LOCAL: u8 = 32;
pub const OPCODE_READ_NAMED: u8 = 33;
pub const OPCODE_WRITE_NAMED: u8 = 34;
pub const OPCODE_TYPEDEF_NAMED: u8 = 35;
pub const OPCODE_READ_INDEX: u8 = 36;
pub const OPCODE_WRITE_INDEX: u8 = 37;
pub const OPCODE_READ_ATTRIBUTE: u8 = 38;
pub const OPCODE_WRITE_ATTRIBUTE: u8 = 39;
pub const OPCODE_READ_UPLEVEL: u8 = 40;
// ...
pub const OPCODE_EQ: u8 = 50;
pub const OPCODE_LT: u8 = 51;
pub const OPCODE_GT: u8 = 52;
pub const OPCODE_LTE: u8 = 53;
pub const OPCODE_GTE: u8 = 54;
pub const OPCODE_ISA: u8 = 55;
pub const OPCODE_LOGICAL_AND: u8 = 56;
pub const OPCODE_LOGICAL_OR: u8 = 57;
pub const OPCODE_XOR: u8 = 58;
pub const OPCODE_NOT: u8 = 59;
pub const OPCODE_BITWISE_AND: u8 = 60;
pub const OPCODE_BITWISE_OR: u8 = 61;
pub const OPCODE_JUMP: u8 = 62;
pub const OPCODE_JUMP_TRUE: u8 = 63;
pub const OPCODE_JUMP_FALSE: u8 = 64;
pub const OPCODE_JUMP_IF_ARG_SUPPLIED: u8 = 65;
// ...
pub const OPCODE_GUARD_ENTER: u8 = 70;
pub const OPCODE_GUARD_EXIT: u8 = 71;
pub const OPCODE_TRY_ENTER: u8 = 72;
pub const OPCODE_TRY_EXIT: u8 = 73;
pub const OPCODE_THROW: u8 = 74;
pub const OPCODE_CALL: u8 = 75;
pub const OPCODE_RETURN: u8 = 76;
// ...
pub const OPCODE_BUILD_LIST: u8 = 80;
pub const OPCODE_BUILD_FUNCTION: u8 = 81;
pub const OPCODE_STORE_UPLEVEL: u8 = 82;
pub const OPCODE_BUILD_STRUCT: u8 = 83;
pub const OPCODE_BUILD_ENUM: u8 = 84;
pub const OPCODE_BUILD_MIXIN: u8 = 85;
pub const OPCODE_BIND_METHOD: u8 = 86;
pub const OPCODE_BIND_CASE: u8 = 87;
pub const OPCODE_INCLUDE_MIXIN: u8 = 88;
pub const OPCODE_NEW_ENUM_VAL: u8 = 89;
pub const OPCODE_ENUM_CHECK_IS_CASE: u8 = 90;
pub const OPCODE_ENUM_EXTRACT_PAYLOAD: u8 = 91;
// ...
pub const OPCODE_IMPORT: u8 = 250;
pub const OPCODE_LIFT_MODULE: u8 = 251;
pub const OPCODE_LOAD_DYLIB: u8 = 252;
pub const OPCODE_ASSERT: u8 = 253;
pub const OPCODE_HALT: u8 = 254;

#[rustfmt::skip]
pub mod function_attribs {
    pub const FUNC_IS_METHOD:            u8 = 1_u8 << 0;
    pub const METHOD_ATTRIBUTE_TYPE:     u8 = 1_u8 << 1;
    pub const FUNC_ACCEPTS_VARARG:       u8 = 1_u8 << 2;
}

#[allow(unused_imports)]
use function_attribs::*;

#[rustfmt::skip]
pub mod runtime_value_ids {
    pub const RUNTIME_VALUE_THIS_MODULE:   u8 = 1;
}

#[allow(unused_imports)]
use runtime_value_ids::*;

#[rustfmt::skip]
pub mod builtin_type_ids {
    pub const BUILTIN_TYPE_ANY:            u8 = 1;
    pub const BUILTIN_TYPE_INT:            u8 = 2;
    pub const BUILTIN_TYPE_LIST:           u8 = 3;
    pub const BUILTIN_TYPE_STRING:         u8 = 4;
    pub const BUILTIN_TYPE_BOOL:           u8 = 5;
    pub const BUILTIN_TYPE_MAYBE:          u8 = 6;
    pub const BUILTIN_TYPE_FLOAT:          u8 = 7;
    pub const BUILTIN_TYPE_UNIMPLEMENTED:  u8 = 8;
    pub const BUILTIN_TYPE_RUNTIME_ERROR:  u8 = 9;
    pub const BUILTIN_TYPE_UNIT:           u8 = 10;
    pub const BUILTIN_TYPE_RESULT:         u8 = 11;
}

#[allow(unused_imports)]
use builtin_type_ids::*;

#[rustfmt::skip]
pub mod enum_case_attribs {
    pub const CASE_HAS_PAYLOAD:            u8 = 1_u8 << 0;
}

#[allow(unused_imports)]
use enum_case_attribs::*;

#[derive(Clone)]
pub enum Opcode {
    Nop,
    Push(u16),
    Push0,
    Push1,
    PushTrue,
    PushFalse,
    PushBuiltinTy(u8),
    PushRuntimeValue(u8),
    Pop,
    Dup,
    Swap,
    Copy(u8),
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg,
    ShiftLeft,
    ShiftRight,
    Not,
    Equal,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    ReadLocal(u8),
    WriteLocal(u8),
    TypedefLocal(u8),
    ReadNamed(u16),
    WriteNamed(u16),
    TypedefNamed(u16),
    ReadIndex,
    WriteIndex,
    ReadAttribute(u16),
    WriteAttribute(u16),
    ReadUplevel(u8),
    LogicalAnd,
    LogicalOr,
    Xor,
    BitwiseAnd,
    BitwiseOr,
    JumpTrue(u16),
    JumpFalse(u16),
    Jump(u16),
    JumpIfArgSupplied(u8, u16),
    Call(u8),
    Return,
    GuardEnter,
    GuardExit,
    TryEnter(u16),
    TryExit,
    Throw,
    BuildList(u32),
    BuildFunction(u8),
    StoreUplevel(u8),
    BuildStruct,
    BuildEnum,
    BuildMixin,
    BindMethod(u8, u16),
    BindCase(u8, u16),
    IncludeMixin,
    NewEnumVal(u16),
    EnumCheckIsCase(u16),
    EnumExtractPayload,
    Isa,
    Import(u16),
    LiftModule,
    LoadDylib(u16),
    Assert(u16),
    Halt,
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nop => write!(f, "NOP"),
            Self::Push(arg0) => write!(f, "PUSH @{arg0}"),
            Self::Push0 => write!(f, "PUSH_0"),
            Self::Push1 => write!(f, "PUSH_1"),
            Self::PushTrue => write!(f, "PUSH_T"),
            Self::PushFalse => write!(f, "PUSH_F"),
            Self::PushBuiltinTy(arg0) => write!(f, "PUSH_BUILTIN_TY {arg0}"),
            Self::PushRuntimeValue(arg0) => write!(f, "PUSH_RUNTIME_VAL {arg0}"),
            Self::Pop => write!(f, "POP"),
            Self::Dup => write!(f, "DUP"),
            Self::Swap => write!(f, "SWAP"),
            Self::Copy(n) => write!(f, "COPY -{n}"),
            Self::Add => write!(f, "ADD"),
            Self::Sub => write!(f, "SUB"),
            Self::Mul => write!(f, "MUL"),
            Self::Div => write!(f, "DIV"),
            Self::Rem => write!(f, "REM"),
            Self::Equal => write!(f, "EQ"),
            Self::Neg => write!(f, "NEG"),
            Self::ShiftLeft => write!(f, "SHL"),
            Self::ShiftRight => write!(f, "SHR"),
            Self::Not => write!(f, "NOT"),
            Self::ReadLocal(arg0) => write!(f, "READ_LOCAL {arg0}"),
            Self::WriteLocal(arg0) => write!(f, "WRITE_LOCAL {arg0}"),
            Self::TypedefLocal(arg0) => write!(f, "TYPEDEF_LOCAL {arg0}"),
            Self::ReadNamed(arg0) => write!(f, "READ_NAMED @{arg0}"),
            Self::WriteNamed(arg0) => write!(f, "WRITE_NAMED @{arg0}"),
            Self::TypedefNamed(arg0) => write!(f, "TYPEDEF_NAMED @{arg0}"),
            Self::ReadIndex => write!(f, "READ_INDEX"),
            Self::WriteIndex => write!(f, "WRITE_INDEX"),
            Self::ReadAttribute(arg0) => write!(f, "READ_ATTRIB @{arg0}"),
            Self::WriteAttribute(arg0) => write!(f, "WRITE_ATTRIB @{arg0}"),
            Self::ReadUplevel(arg0) => write!(f, "READ_UPLEVEL {arg0}"),
            Self::LogicalAnd => write!(f, "ANDL"),
            Self::LogicalOr => write!(f, "ORL"),
            Self::Xor => write!(f, "XOR"),
            Self::BitwiseAnd => write!(f, "ANDB"),
            Self::BitwiseOr => write!(f, "ORB"),
            Self::LessThan => write!(f, "LT"),
            Self::GreaterThan => write!(f, "GT"),
            Self::LessThanEqual => write!(f, "LTE"),
            Self::GreaterThanEqual => write!(f, "GTE"),
            Self::JumpTrue(arg0) => write!(f, "JUMP_TRUE {arg0}"),
            Self::JumpFalse(arg0) => write!(f, "JUMP_FALSE {arg0}"),
            Self::Jump(arg0) => write!(f, "JUMP {arg0}"),
            Self::JumpIfArgSupplied(arg0, arg1) => write!(f, "JUMP_IF_ARG_SUPPLIED {arg0} {arg1}"),
            Self::Call(arg0) => write!(f, "CALL {arg0}"),
            Self::Return => write!(f, "RETURN"),
            Self::GuardEnter => write!(f, "ENTER_GUARD"),
            Self::GuardExit => write!(f, "EXIT_GUARD"),
            Self::TryEnter(arg0) => write!(f, "ENTER_TRY {arg0}"),
            Self::TryExit => write!(f, "EXIT_TRY"),
            Self::Throw => write!(f, "THROW"),
            Self::BuildList(arg0) => write!(f, "BUILD_LIST {arg0}"),
            Self::BuildFunction(arg0) => write!(f, "BUILD_FUNC {arg0}"),
            Self::StoreUplevel(arg0) => write!(f, "STORE_UPLEVEL {arg0}"),
            Self::BuildStruct => write!(f, "BUILD_STRUCT"),
            Self::BuildEnum => write!(f, "BUILD_ENUM"),
            Self::BuildMixin => write!(f, "BUILD_MIXIN"),
            Self::BindMethod(arg0, arg1) => write!(f, "BIND_M {arg0} @{arg1}"),
            Self::BindCase(arg0, arg1) => write!(f, "BIND_C {arg0} @{arg1}"),
            Self::IncludeMixin => write!(f, "INCLUDE_MIXIN"),
            Self::NewEnumVal(arg0) => write!(f, "NEW_ENUM_VAL @{arg0}"),
            Self::EnumCheckIsCase(arg0) => write!(f, "ENUM_CHECK_IS_CASE @{arg0}"),
            Self::EnumExtractPayload => write!(f, "ENUM_EXTRACT_PAYLOAD"),
            Self::Isa => write!(f, "ISA"),
            Self::Import(arg0) => write!(f, "IMPORT @{arg0}"),
            Self::LiftModule => write!(f, "LIFT_MODULE"),
            Self::LoadDylib(arg0) => write!(f, "LOAD_DYLIB @{arg0}"),
            Self::Assert(arg0) => write!(f, "ASSERT @{arg0}"),
            Self::Halt => write!(f, "HALT"),
        }
    }
}

impl Opcode {
    pub fn byte_size(&self) -> usize {
        match self {
            Self::Nop => 1,
            Self::Push(_) => 3,
            Self::Push0 => 1,
            Self::Push1 => 1,
            Self::PushTrue => 1,
            Self::PushFalse => 1,
            Self::PushBuiltinTy(_) => 2,
            Self::PushRuntimeValue(_) => 2,
            Self::Pop => 1,
            Self::Dup => 1,
            Self::Swap => 1,
            Self::Copy(_) => 2,
            Self::Add => 1,
            Self::Sub => 1,
            Self::Mul => 1,
            Self::Div => 1,
            Self::Rem => 1,
            Self::Equal => 1,
            Self::Neg => 1,
            Self::ShiftLeft => 1,
            Self::ShiftRight => 1,
            Self::Not => 1,
            Self::ReadLocal(_) => 2,
            Self::WriteLocal(_) => 2,
            Self::TypedefLocal(_) => 2,
            Self::ReadNamed(_) => 3,
            Self::WriteNamed(_) => 3,
            Self::TypedefNamed(_) => 3,
            Self::ReadIndex => 1,
            Self::WriteIndex => 1,
            Self::ReadAttribute(_) => 3,
            Self::WriteAttribute(_) => 3,
            Self::ReadUplevel(_) => 2,
            Self::LogicalAnd => 1,
            Self::LogicalOr => 1,
            Self::Xor => 1,
            Self::BitwiseAnd => 1,
            Self::BitwiseOr => 1,
            Self::GreaterThan => 1,
            Self::LessThan => 1,
            Self::GreaterThanEqual => 1,
            Self::LessThanEqual => 1,
            Self::JumpTrue(_) => 3,
            Self::JumpFalse(_) => 3,
            Self::JumpIfArgSupplied(..) => 4,
            Self::Jump(_) => 3,
            Self::Call(_) => 2,
            Self::Return => 1,
            Self::GuardEnter => 1,
            Self::GuardExit => 1,
            Self::TryEnter(_) => 3,
            Self::TryExit => 1,
            Self::Throw => 1,
            Self::BuildList(_) => 5,
            Self::BuildFunction(_) => 2,
            Self::StoreUplevel(_) => 2,
            Self::BuildStruct => 1,
            Self::BuildEnum => 1,
            Self::BuildMixin => 1,
            Self::BindMethod(..) => 4,
            Self::BindCase(..) => 4,
            Self::IncludeMixin => 1,
            Self::NewEnumVal(_) => 3,
            Self::EnumCheckIsCase(_) => 3,
            Self::EnumExtractPayload => 1,
            Self::Isa => 1,
            Self::Import(_) => 3,
            Self::LiftModule => 1,
            Self::LoadDylib(_) => 3,
            Self::Assert(_) => 3,
            Self::Halt => 1,
        }
    }
}
