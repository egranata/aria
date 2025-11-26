// SPDX-License-Identifier: Apache-2.0
use std::{cell::RefCell, collections::HashSet, rc::Rc};

use aria_parser::ast::SourcePointer;
use haxby_opcodes::Opcode;

use crate::{
    CompilationOptions, bc_writer::BytecodeWriter, constant_value::ConstantValues,
    line_table::LineTable,
};

pub enum BasicBlockOpcode {
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
    ReadLocal(u8),
    WriteLocal(u8),
    TypedefLocal(u8),
    ReadNamed(u16),
    WriteNamed(u16),
    TypedefNamed(u16),
    ReadIndex(u8),
    WriteIndex(u8),
    ReadAttribute(u16),
    WriteAttribute(u16),
    ReadUplevel(u8),
    LogicalAnd,
    BitwiseAnd,
    LogicalOr,
    BitwiseOr,
    Xor,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    JumpTrue(Rc<BasicBlock>),
    JumpFalse(Rc<BasicBlock>),
    Jump(Rc<BasicBlock>),
    JumpIfArgSupplied(u8, Rc<BasicBlock>),
    Call(u8),
    Return,
    TryEnter(Rc<BasicBlock>),
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
    TryUnwrapProtocol(u8),
    Isa,
    Import(u16),
    LiftModule,
    LoadDylib(u16),
    Assert(u16),
    Halt,
}

impl BasicBlockOpcode {
    pub fn is_terminal(&self) -> bool {
        match self {
            Self::Nop => false,
            Self::Push(_) => false,
            Self::Push0 => false,
            Self::Push1 => false,
            Self::PushTrue => false,
            Self::PushFalse => false,
            Self::PushBuiltinTy(_) => false,
            Self::PushRuntimeValue(_) => false,
            Self::Pop => false,
            Self::Dup => false,
            Self::Swap => false,
            Self::Copy(_) => false,
            Self::Add => false,
            Self::Sub => false,
            Self::Mul => false,
            Self::Div => false,
            Self::Rem => false,
            Self::Neg => false,
            Self::ShiftLeft => false,
            Self::ShiftRight => false,
            Self::Not => false,
            Self::Equal => false,
            Self::ReadLocal(_) => false,
            Self::WriteLocal(_) => false,
            Self::TypedefLocal(_) => false,
            Self::ReadNamed(_) => false,
            Self::WriteNamed(_) => false,
            Self::TypedefNamed(_) => false,
            Self::ReadIndex(_) => false,
            Self::WriteIndex(_) => false,
            Self::ReadAttribute(_) => false,
            Self::WriteAttribute(_) => false,
            Self::ReadUplevel(_) => false,
            Self::LogicalAnd => false,
            Self::LogicalOr => false,
            Self::Xor => false,
            Self::BitwiseAnd => false,
            Self::BitwiseOr => false,
            Self::LessThan => false,
            Self::GreaterThan => false,
            Self::LessThanEqual => false,
            Self::GreaterThanEqual => false,
            Self::JumpTrue(_) => false,
            Self::JumpFalse(_) => false,
            Self::Jump(_) => true,
            Self::JumpIfArgSupplied(..) => false,
            Self::Call(_) => false,
            Self::Return => true,
            Self::TryEnter(_) => false,
            Self::TryExit => false,
            Self::Throw => true,
            Self::BuildList(_) => false,
            Self::BuildFunction(_) => false,
            Self::StoreUplevel(_) => false,
            Self::BuildStruct => false,
            Self::BuildEnum => false,
            Self::BuildMixin => false,
            Self::BindMethod(..) => false,
            Self::BindCase(..) => false,
            Self::IncludeMixin => false,
            Self::NewEnumVal(_) => false,
            Self::EnumCheckIsCase(_) => false,
            Self::EnumExtractPayload => false,
            Self::TryUnwrapProtocol(_) => false,
            Self::Isa => false,
            Self::Import(_) => false,
            Self::LiftModule => false,
            Self::LoadDylib(_) => false,
            Self::Assert(_) => false,
            Self::Halt => true,
        }
    }

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
            Self::Neg => 1,
            Self::ShiftLeft => 1,
            Self::ShiftRight => 1,
            Self::Not => 1,
            Self::Equal => 1,
            Self::ReadLocal(_) => 2,
            Self::WriteLocal(_) => 2,
            Self::TypedefLocal(_) => 2,
            Self::ReadNamed(_) => 3,
            Self::WriteNamed(_) => 3,
            Self::TypedefNamed(_) => 3,
            Self::ReadIndex(_) => 2,
            Self::WriteIndex(_) => 2,
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
            Self::Jump(_) => 3,
            Self::JumpIfArgSupplied(..) => 4,
            Self::Call(_) => 2,
            Self::Return => 1,
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
            Self::TryUnwrapProtocol(_) => 2,
            Self::Isa => 1,
            Self::Import(_) => 3,
            Self::LiftModule => 1,
            Self::LoadDylib(_) => 3,
            Self::Assert(_) => 3,
            Self::Halt => 1,
        }
    }

    pub fn is_jump_instruction(&self) -> Option<Rc<BasicBlock>> {
        match self {
            Self::TryEnter(dst)
            | Self::JumpIfArgSupplied(_, dst)
            | Self::Jump(dst)
            | Self::JumpTrue(dst)
            | Self::JumpFalse(dst) => Some(dst.clone()),
            _ => None,
        }
    }

    pub fn to_opcodes(&self, parent: &FunctionBuilder) -> Vec<Opcode> {
        match self {
            Self::Nop => vec![Opcode::Nop],
            Self::Push(v) => vec![Opcode::Push(*v)],
            Self::Push0 => vec![Opcode::Push0],
            Self::Push1 => vec![Opcode::Push1],
            Self::PushTrue => vec![Opcode::PushTrue],
            Self::PushFalse => vec![Opcode::PushFalse],
            Self::PushBuiltinTy(n) => vec![Opcode::PushBuiltinTy(*n)],
            Self::PushRuntimeValue(n) => vec![Opcode::PushRuntimeValue(*n)],
            Self::Pop => vec![Opcode::Pop],
            Self::Dup => vec![Opcode::Dup],
            Self::Swap => vec![Opcode::Swap],
            Self::Copy(n) => vec![Opcode::Copy(*n)],
            Self::Add => vec![Opcode::Add],
            Self::Sub => vec![Opcode::Sub],
            Self::Mul => vec![Opcode::Mul],
            Self::Div => vec![Opcode::Div],
            Self::Rem => vec![Opcode::Rem],
            Self::Neg => vec![Opcode::Neg],
            Self::ShiftLeft => vec![Opcode::ShiftLeft],
            Self::ShiftRight => vec![Opcode::ShiftRight],
            Self::Not => vec![Opcode::Not],
            Self::Equal => vec![Opcode::Equal],
            Self::ReadLocal(n) => vec![Opcode::ReadLocal(*n)],
            Self::WriteLocal(n) => vec![Opcode::WriteLocal(*n)],
            Self::TypedefLocal(n) => vec![Opcode::TypedefLocal(*n)],
            Self::ReadNamed(n) => vec![Opcode::ReadNamed(*n)],
            Self::WriteNamed(n) => vec![Opcode::WriteNamed(*n)],
            Self::TypedefNamed(n) => vec![Opcode::TypedefNamed(*n)],
            Self::ReadIndex(n) => vec![Opcode::ReadIndex(*n)],
            Self::WriteIndex(n) => vec![Opcode::WriteIndex(*n)],
            Self::ReadAttribute(n) => vec![Opcode::ReadAttribute(*n)],
            Self::WriteAttribute(n) => vec![Opcode::WriteAttribute(*n)],
            Self::ReadUplevel(n) => vec![Opcode::ReadUplevel(*n)],
            Self::LogicalAnd => vec![Opcode::LogicalAnd],
            Self::LogicalOr => vec![Opcode::LogicalOr],
            Self::Xor => vec![Opcode::Xor],
            Self::BitwiseAnd => vec![Opcode::BitwiseAnd],
            Self::BitwiseOr => vec![Opcode::BitwiseOr],
            Self::GreaterThan => vec![Opcode::GreaterThan],
            Self::LessThan => vec![Opcode::LessThan],
            Self::GreaterThanEqual => vec![Opcode::GreaterThanEqual],
            Self::LessThanEqual => vec![Opcode::LessThanEqual],
            Self::JumpTrue(dst) => {
                let offset = parent.offset_of_block(dst).expect("invalid block") - 1;
                vec![Opcode::JumpTrue(offset)]
            }
            Self::JumpFalse(dst) => {
                let offset = parent.offset_of_block(dst).expect("invalid block") - 1;
                vec![Opcode::JumpFalse(offset)]
            }
            Self::Jump(dst) => {
                let offset = parent.offset_of_block(dst).expect("invalid block") - 1;
                vec![Opcode::Jump(offset)]
            }
            Self::JumpIfArgSupplied(arg, dst) => {
                let offset = parent.offset_of_block(dst).expect("invalid block") - 1;
                vec![Opcode::JumpIfArgSupplied(*arg, offset)]
            }
            Self::Call(n) => vec![Opcode::Call(*n)],
            Self::Return => vec![Opcode::Return],
            Self::TryEnter(dst) => {
                let offset = parent.offset_of_block(dst).expect("invalid block") - 1;
                vec![Opcode::TryEnter(offset)]
            }
            Self::TryExit => vec![Opcode::TryExit],
            Self::Throw => vec![Opcode::Throw],
            Self::BuildList(v) => vec![Opcode::BuildList(*v)],
            Self::BuildFunction(a) => vec![Opcode::BuildFunction(*a)],
            Self::StoreUplevel(a) => vec![Opcode::StoreUplevel(*a)],
            Self::BuildStruct => vec![Opcode::BuildStruct],
            Self::BuildEnum => vec![Opcode::BuildEnum],
            Self::BuildMixin => vec![Opcode::BuildMixin],
            Self::BindMethod(x, y) => vec![Opcode::BindMethod(*x, *y)],
            Self::BindCase(x, y) => vec![Opcode::BindCase(*x, *y)],
            Self::IncludeMixin => vec![Opcode::IncludeMixin],
            Self::NewEnumVal(v) => vec![Opcode::NewEnumVal(*v)],
            Self::EnumCheckIsCase(v) => vec![Opcode::EnumCheckIsCase(*v)],
            Self::EnumExtractPayload => vec![Opcode::EnumExtractPayload],
            Self::TryUnwrapProtocol(v) => vec![Opcode::TryUnwrapProtocol(*v)],
            Self::Isa => vec![Opcode::Isa],
            Self::Import(v) => vec![Opcode::Import(*v)],
            Self::LiftModule => vec![Opcode::LiftModule],
            Self::LoadDylib(n) => vec![Opcode::LoadDylib(*n)],
            Self::Assert(v) => vec![Opcode::Assert(*v)],
            Self::Halt => vec![Opcode::Halt],
        }
    }
}

struct BasicBlockEntry {
    op: BasicBlockOpcode,
    src: Option<SourcePointer>,
}

impl From<BasicBlockOpcode> for BasicBlockEntry {
    fn from(op: BasicBlockOpcode) -> Self {
        Self { op, src: None }
    }
}

impl BasicBlockEntry {
    fn byte_size(&self) -> usize {
        self.op.byte_size()
    }

    fn to_opcodes(&self, parent: &FunctionBuilder) -> Vec<Opcode> {
        self.op.to_opcodes(parent)
    }
}

pub struct BasicBlock {
    name: String,
    id: usize,
    writer: RefCell<Vec<BasicBlockEntry>>,
}

struct LocalValuesAccess {
    reads: HashSet<u8>,
    writes: HashSet<u8>,
}

impl LocalValuesAccess {
    fn calculate_unused_locals(&self) -> HashSet<u8> {
        self.writes.difference(&self.reads).cloned().collect()
    }
}

impl BasicBlock {
    fn new(name: &str, id: usize) -> Self {
        Self {
            name: name.to_owned(),
            id,
            writer: RefCell::new(Default::default()),
        }
    }

    #[deprecated(note = "use write_opcode_and_source_info")]
    pub fn write_opcode(&self, op: BasicBlockOpcode) -> &Self {
        self.writer.borrow_mut().push(op.into());
        self
    }

    pub fn write_opcode_and_source_info(&self, op: BasicBlockOpcode, src: SourcePointer) -> &Self {
        let bbe = BasicBlockEntry { op, src: Some(src) };
        self.writer.borrow_mut().push(bbe);
        self
    }

    pub fn len(&self) -> usize {
        self.writer.borrow().len()
    }

    pub fn byte_size(&self) -> usize {
        self.writer
            .borrow()
            .iter()
            .map(|o| o.byte_size())
            .sum::<usize>()
    }

    pub fn is_empty(&self) -> bool {
        self.writer.borrow().is_empty()
    }

    pub fn is_terminal(&self) -> bool {
        let br = self.writer.borrow();
        for src_op in br.as_slice() {
            if src_op.op.is_terminal() {
                return true;
            }
        }

        false
    }

    fn replace_double_jump(&self) -> bool {
        let mut any = false;

        let mut br = self.writer.borrow_mut();
        for i in 0..br.len() {
            if let BasicBlockOpcode::Jump(dest) = &br[i].op {
                let dest = dest.clone();
                if dest.id == self.id {
                    continue;
                }
                if dest.is_empty() {
                    continue;
                }
                let dest_br = dest.writer.borrow();
                if let BasicBlockOpcode::Jump(final_dest) = &dest_br[0].op {
                    br[i].op = BasicBlockOpcode::Jump(final_dest.clone());
                    any = true;
                }
            }
        }

        any
    }

    fn optimize_true_false(&self, cv: &ConstantValues) {
        let mut br = self.writer.borrow_mut();
        for i in 0..br.len() {
            if let BasicBlockOpcode::ReadNamed(idx) = &br[i].op
                && let Some(crate::constant_value::ConstantValue::String(x)) = cv.get(*idx as usize)
            {
                if x == "true" {
                    br[i].op = BasicBlockOpcode::PushTrue;
                } else if x == "false" {
                    br[i].op = BasicBlockOpcode::PushFalse;
                }
            }
        }
    }

    fn optimize_redundant_conditional_jumps(&self) {
        let mut br = self.writer.borrow_mut();
        let mut i = 0;
        while i + 1 < br.len() {
            match (&br[i].op, &br[i + 1].op) {
                (BasicBlockOpcode::PushTrue, BasicBlockOpcode::JumpTrue(target))
                | (BasicBlockOpcode::PushFalse, BasicBlockOpcode::JumpFalse(target)) => {
                    br[i].op = BasicBlockOpcode::Jump(target.clone());
                    br.remove(i + 1);
                    continue;
                }
                (BasicBlockOpcode::PushTrue, BasicBlockOpcode::JumpFalse(_))
                | (BasicBlockOpcode::PushFalse, BasicBlockOpcode::JumpTrue(_)) => {
                    br[i].op = BasicBlockOpcode::Nop;
                    br.remove(i + 1);
                    continue;
                }
                _ => {}
            }

            i += 1;
        }
    }

    fn remove_instructions_after_terminal(&self) {
        let mut br = self.writer.borrow_mut();
        for i in 0..br.len() {
            if br[i].op.is_terminal() {
                while br.len() != i + 1 {
                    br.remove(i + 1);
                }
                return;
            }
        }
    }

    fn remove_redundant_local_reads(&self) {
        let mut br = self.writer.borrow_mut();
        if br.len() < 2 {
            return;
        }

        for i in 0..br.len() - 1 {
            if let BasicBlockOpcode::ReadLocal(x) = br[i].op {
                let mut j = i + 1;
                while j < br.len() {
                    match br[j].op {
                        BasicBlockOpcode::ReadLocal(y) => {
                            if x != y {
                                break;
                            } else {
                                br[j].op = BasicBlockOpcode::Dup;
                            }
                        }
                        _ => {
                            break;
                        }
                    }
                    j += 1;
                }
            }
        }
    }

    fn remove_redundant_named_reads(&self) {
        let mut br = self.writer.borrow_mut();
        if br.len() < 2 {
            return;
        }

        for i in 0..br.len() - 1 {
            if let BasicBlockOpcode::ReadNamed(x) = br[i].op {
                let mut j = i + 1;
                while j < br.len() {
                    match br[j].op {
                        BasicBlockOpcode::ReadNamed(y) => {
                            if x != y {
                                break;
                            } else {
                                br[j].op = BasicBlockOpcode::Dup;
                            }
                        }
                        _ => {
                            break;
                        }
                    }
                    j += 1;
                }
            }
        }
    }

    fn remove_store_load_sequence(&self) {
        let mut br = self.writer.borrow_mut();
        if br.len() < 2 {
            return;
        }

        for i in 0..br.len() - 1 {
            if let BasicBlockOpcode::WriteLocal(x) = br[i].op
                && let BasicBlockOpcode::ReadLocal(y) = br[i + 1].op
                && x == y
            {
                br[i].op = BasicBlockOpcode::Dup;
                br[i + 1].op = BasicBlockOpcode::WriteLocal(x);
            }
        }
    }

    fn remove_nop_instructions(&self) {
        let mut br = self.writer.borrow_mut();
        br.retain(|x| !matches!(x.op, BasicBlockOpcode::Nop));
    }

    fn remove_push_pop_pairs(&self) {
        let mut br = self.writer.borrow_mut();
        if br.len() < 2 {
            return;
        }

        for i in 0..br.len() - 1 {
            if let (
                BasicBlockOpcode::Push0
                | BasicBlockOpcode::Push1
                | BasicBlockOpcode::PushFalse
                | BasicBlockOpcode::PushTrue
                | BasicBlockOpcode::Push(_)
                | BasicBlockOpcode::PushBuiltinTy(_)
                | BasicBlockOpcode::Dup,
                BasicBlockOpcode::Pop,
            ) = (&br[i].op, &br[i + 1].op)
            {
                br[i].op = BasicBlockOpcode::Nop;
                br[i + 1].op = BasicBlockOpcode::Nop;
            }
        }
    }

    fn run_optimize_passes(&self, cv: &ConstantValues) {
        self.optimize_true_false(cv);
        self.optimize_redundant_conditional_jumps();
        self.remove_redundant_local_reads();
        self.remove_redundant_named_reads();
        self.remove_store_load_sequence();
        self.remove_instructions_after_terminal();
        self.remove_nop_instructions();
        self.remove_push_pop_pairs();
        self.remove_nop_instructions();
        while self.replace_double_jump() {}
    }

    fn drop_unused_locals(&self, values: &HashSet<u8>) {
        let mut br = self.writer.borrow_mut();

        for i in 0..br.len() {
            match br[i].op {
                BasicBlockOpcode::ReadLocal(x) => {
                    assert!(!values.contains(&x));
                }
                BasicBlockOpcode::TypedefLocal(x) => {
                    if values.contains(&x) {
                        br[i].op = BasicBlockOpcode::Pop;
                    }
                }
                BasicBlockOpcode::WriteLocal(x) => {
                    if values.contains(&x) {
                        br[i].op = BasicBlockOpcode::Pop;
                    }
                }
                _ => {}
            }
        }
    }

    fn calculate_locals_access(&self, dest: &mut LocalValuesAccess) {
        let br = self.writer.borrow();
        for i in 0..br.len() {
            match br[i].op {
                BasicBlockOpcode::ReadLocal(x) | BasicBlockOpcode::StoreUplevel(x) => {
                    dest.reads.insert(x);
                }
                BasicBlockOpcode::WriteLocal(x) => {
                    dest.writes.insert(x);
                }
                BasicBlockOpcode::TypedefLocal(x) => {
                    if i > 0 {
                        if let BasicBlockOpcode::PushBuiltinTy(x) = br[i - 1].op
                            && x == 1
                        {
                            dest.writes.insert(x);
                        } else {
                            dest.reads.insert(x);
                            dest.writes.insert(x);
                        }
                    } else {
                        // this is quite odd, as there would have to be something else defining
                        // the type of the local on the stack, but just keep going for sake of completeness
                        dest.writes.insert(x);
                    }
                }
                _ => {}
            }
        }
    }

    fn write(&self, parent: &FunctionBuilder, dest: &mut BytecodeWriter) {
        let br = self.writer.borrow();
        for src_op in br.as_slice() {
            for dst_op in src_op.to_opcodes(parent) {
                dest.write_opcode(&dst_op);
            }
        }
    }

    fn write_line_table(&self, parent: &FunctionBuilder, offset: u16, line_table: &LineTable) {
        let mut cur_offset = offset;
        let br = self.writer.borrow();
        for src_op in br.as_slice() {
            for dst_op in src_op.to_opcodes(parent) {
                if let Some(src) = &src_op.src {
                    line_table.insert(cur_offset - 1_u16, src.clone());
                }
                cur_offset += dst_op.byte_size() as u16;
            }
        }
    }
}

pub struct FunctionBuilder {
    blocks: Vec<Rc<BasicBlock>>,
    names: HashSet<String>,
    current: Rc<BasicBlock>,
    bb_id: usize,
    line_table: LineTable,
}

impl Default for FunctionBuilder {
    fn default() -> Self {
        let mut this = Self {
            blocks: Vec::new(),
            names: HashSet::new(),
            current: Rc::new(BasicBlock::new("entry", 0)),
            bb_id: 1,
            line_table: Default::default(),
        };
        this.blocks.push(this.current.clone());
        this.names.insert(this.current.name.clone());
        this
    }
}

impl FunctionBuilder {
    pub fn try_get_block(&self, name: &str) -> Option<Rc<BasicBlock>> {
        for blk in &self.blocks {
            if blk.name == name {
                return Some(blk.clone());
            }
        }

        None
    }

    pub fn get_block(&self, name: &str) -> Rc<BasicBlock> {
        self.try_get_block(name).expect("block is missing")
    }

    fn uniq_name(&self, name: &str) -> String {
        let mut name = String::from(name);
        while self.names.contains(&name) {
            name += "_";
        }

        name
    }

    fn make_new_block(&mut self, name: &str) -> Rc<BasicBlock> {
        let blk = Rc::new(BasicBlock::new(name, self.bb_id));
        self.bb_id += 1;
        blk
    }

    pub fn insert_block_after(&mut self, name: &str, target: &Rc<BasicBlock>) -> Rc<BasicBlock> {
        let name = self.uniq_name(name);
        let blk = self.make_new_block(&name);
        let mut inserted = false;

        for i in 0..self.blocks.len() {
            let blk_i = &self.blocks[i];
            if blk_i.id == target.id {
                if i + 1 >= self.blocks.len() {
                    self.blocks.push(blk.clone());
                } else {
                    self.blocks.insert(i + 1, blk.clone());
                }
                inserted = true;
                break;
            }
        }

        if !inserted {
            self.blocks.push(blk.clone());
        }

        self.names.insert(name);
        blk
    }

    pub fn append_block_at_end(&mut self, name: &str) -> Rc<BasicBlock> {
        let name = self.uniq_name(name);
        let blk = self.make_new_block(&name);

        self.blocks.push(blk.clone());
        self.names.insert(name);
        blk
    }

    pub fn set_current_block(&mut self, blk: Rc<BasicBlock>) {
        self.current = blk;
    }

    pub fn get_current_block(&self) -> Rc<BasicBlock> {
        self.current.clone()
    }

    pub fn offset_of_block(&self, blk: &Rc<BasicBlock>) -> Option<u16> {
        let mut count = 0;
        for next in &self.blocks {
            if Rc::ptr_eq(next, blk) {
                return Some((count + 1) as u16);
            } else {
                count += next.byte_size();
            }
        }
        None
    }

    fn find_orphaned_blocks(&self) -> HashSet<usize> {
        let mut orphans = HashSet::<usize>::default();

        for blk in &self.blocks {
            if blk.id != 0 {
                orphans.insert(blk.id);
            }
        }

        for blk in &self.blocks {
            let br = blk.writer.borrow();
            for src_op in br.as_slice() {
                if let Some(dst) = src_op.op.is_jump_instruction() {
                    orphans.remove(&dst.id);
                }
            }
        }

        orphans
    }

    fn remove_block_with_id(&mut self, id: usize) -> bool {
        for i in 0..self.blocks.len() {
            if self.blocks[i].id == id {
                self.blocks.remove(i);
                return true;
            }
        }

        false
    }

    fn run_optimize_passes(&mut self, cv: &ConstantValues) {
        let orphans = self.find_orphaned_blocks();
        for orphan_id in &orphans {
            assert!(self.remove_block_with_id(*orphan_id));
        }

        let locals_access = self.calculate_locals_access();
        let unused_locals = locals_access.calculate_unused_locals();

        for blk in &self.blocks {
            if !unused_locals.is_empty() {
                blk.drop_unused_locals(&unused_locals);
            }
            blk.run_optimize_passes(cv);
        }
    }

    fn calculate_locals_access(&self) -> LocalValuesAccess {
        let mut dest = LocalValuesAccess {
            reads: HashSet::new(),
            writes: HashSet::new(),
        };

        for blk in &self.blocks {
            blk.calculate_locals_access(&mut dest);
        }

        dest
    }

    pub fn write(
        &mut self,
        cv: &ConstantValues,
        options: &CompilationOptions,
    ) -> Result<Vec<u8>, crate::do_compile::CompilationErrorReason> {
        if options.optimize {
            self.run_optimize_passes(cv);
        }

        let mut dest = BytecodeWriter::default();
        for blk in &self.blocks {
            assert!(blk.is_empty() || blk.is_terminal());
            blk.write(self, &mut dest);
        }

        let ret = dest.get_data();
        if ret.len() >= u16::MAX.into() {
            Err(crate::do_compile::CompilationErrorReason::FunctionBodyTooLarge)
        } else {
            Ok(ret)
        }
    }

    pub fn write_line_table(&self) -> &LineTable {
        for blk in &self.blocks {
            blk.write_line_table(self, self.offset_of_block(blk).unwrap(), &self.line_table);
        }

        &self.line_table
    }
}
