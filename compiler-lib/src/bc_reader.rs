// SPDX-License-Identifier: Apache-2.0
use haxby_opcodes::Opcode;

pub struct BytecodeReader {
    data: Vec<u8>,
    idx: usize,
}

impl From<&[u8]> for BytecodeReader {
    fn from(value: &[u8]) -> Self {
        Self {
            data: value.to_vec(),
            idx: 0,
        }
    }
}

pub enum DecodeError {
    EndOfStream,
    InsufficientData,
    UnknownOpcode(u8),
}

pub type DecodeResult<T> = Result<T, DecodeError>;

impl BytecodeReader {
    fn read_u8(&mut self) -> DecodeResult<u8> {
        if self.idx < self.data.len() {
            let val = self.data[self.idx];
            self.idx += 1;
            Ok(val)
        } else {
            Err(DecodeError::EndOfStream)
        }
    }

    fn read_u16(&mut self) -> DecodeResult<u16> {
        Ok(u16::from_le_bytes([self.read_u8()?, self.read_u8()?]))
    }

    fn read_u32(&mut self) -> DecodeResult<u32> {
        Ok(u32::from_le_bytes([
            self.read_u8()?,
            self.read_u8()?,
            self.read_u8()?,
            self.read_u8()?,
        ]))
    }

    pub fn jump_to_index(&mut self, idx: usize) {
        self.idx = idx;
    }

    pub fn get_index(&self) -> usize {
        self.idx
    }

    pub fn read_opcode(&mut self) -> DecodeResult<Opcode> {
        let next = match self.read_u8() {
            Ok(next) => next,
            Err(err) => match err {
                DecodeError::EndOfStream => {
                    return Err(DecodeError::EndOfStream);
                }
                DecodeError::InsufficientData => {
                    return Err(DecodeError::EndOfStream);
                }
                DecodeError::UnknownOpcode(x) => {
                    return Err(DecodeError::UnknownOpcode(x));
                }
            },
        };
        match next {
            haxby_opcodes::OPCODE_NOP => Ok(Opcode::Nop),
            haxby_opcodes::OPCODE_PUSH => self
                .read_u16()
                .map_or(Err(DecodeError::InsufficientData), |b| Ok(Opcode::Push(b))),
            haxby_opcodes::OPCODE_PUSH_0 => Ok(Opcode::Push0),
            haxby_opcodes::OPCODE_PUSH_1 => Ok(Opcode::Push1),
            haxby_opcodes::OPCODE_PUSH_TRUE => Ok(Opcode::PushTrue),
            haxby_opcodes::OPCODE_PUSH_FALSE => Ok(Opcode::PushFalse),
            haxby_opcodes::OPCODE_PUSH_BUILTIN_TYPE => self
                .read_u8()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::PushBuiltinTy(b))
                }),
            haxby_opcodes::OPCODE_PUSH_RUNTIME_VALUE => self
                .read_u8()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::PushRuntimeValue(b))
                }),
            haxby_opcodes::OPCODE_POP => Ok(Opcode::Pop),
            haxby_opcodes::OPCODE_DUP => Ok(Opcode::Dup),
            haxby_opcodes::OPCODE_SWAP => Ok(Opcode::Swap),
            haxby_opcodes::OPCODE_COPY => self
                .read_u8()
                .map_or(Err(DecodeError::InsufficientData), |b| Ok(Opcode::Copy(b))),
            haxby_opcodes::OPCODE_ADD => Ok(Opcode::Add),
            haxby_opcodes::OPCODE_SUB => Ok(Opcode::Sub),
            haxby_opcodes::OPCODE_MUL => Ok(Opcode::Mul),
            haxby_opcodes::OPCODE_DIV => Ok(Opcode::Div),
            haxby_opcodes::OPCODE_REM => Ok(Opcode::Rem),
            haxby_opcodes::OPCODE_EQ => Ok(Opcode::Equal),
            haxby_opcodes::OPCODE_GT => Ok(Opcode::GreaterThan),
            haxby_opcodes::OPCODE_LT => Ok(Opcode::LessThan),
            haxby_opcodes::OPCODE_GTE => Ok(Opcode::GreaterThanEqual),
            haxby_opcodes::OPCODE_LTE => Ok(Opcode::LessThanEqual),
            haxby_opcodes::OPCODE_NEG => Ok(Opcode::Neg),
            haxby_opcodes::OPCODE_SHL => Ok(Opcode::ShiftLeft),
            haxby_opcodes::OPCODE_SHR => Ok(Opcode::ShiftRight),
            haxby_opcodes::OPCODE_NOT => Ok(Opcode::Not),
            haxby_opcodes::OPCODE_READ_LOCAL => self
                .read_u8()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::ReadLocal(b))
                }),
            haxby_opcodes::OPCODE_WRITE_LOCAL => self
                .read_u8()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::WriteLocal(b))
                }),
            haxby_opcodes::OPCODE_TYPEDEF_LOCAL => self
                .read_u8()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::TypedefLocal(b))
                }),
            haxby_opcodes::OPCODE_READ_NAMED => self
                .read_u16()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::ReadNamed(b))
                }),
            haxby_opcodes::OPCODE_WRITE_NAMED => self
                .read_u16()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::WriteNamed(b))
                }),
            haxby_opcodes::OPCODE_TYPEDEF_NAMED => self
                .read_u16()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::TypedefNamed(b))
                }),
            haxby_opcodes::OPCODE_READ_INDEX => Ok(Opcode::ReadIndex),
            haxby_opcodes::OPCODE_WRITE_INDEX => Ok(Opcode::WriteIndex),
            haxby_opcodes::OPCODE_READ_ATTRIBUTE => self
                .read_u16()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::ReadAttribute(b))
                }),
            haxby_opcodes::OPCODE_WRITE_ATTRIBUTE => self
                .read_u16()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::WriteAttribute(b))
                }),
            haxby_opcodes::OPCODE_READ_UPLEVEL => self
                .read_u8()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::ReadUplevel(b))
                }),
            haxby_opcodes::OPCODE_LOGICAL_AND => Ok(Opcode::LogicalAnd),
            haxby_opcodes::OPCODE_LOGICAL_OR => Ok(Opcode::LogicalOr),
            haxby_opcodes::OPCODE_XOR => Ok(Opcode::Xor),
            haxby_opcodes::OPCODE_BITWISE_AND => Ok(Opcode::BitwiseAnd),
            haxby_opcodes::OPCODE_BITWISE_OR => Ok(Opcode::BitwiseOr),
            haxby_opcodes::OPCODE_JUMP_TRUE => self
                .read_u16()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::JumpTrue(b))
                }),
            haxby_opcodes::OPCODE_JUMP_FALSE => self
                .read_u16()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::JumpFalse(b))
                }),
            haxby_opcodes::OPCODE_JUMP => self
                .read_u16()
                .map_or(Err(DecodeError::InsufficientData), |b| Ok(Opcode::Jump(b))),
            haxby_opcodes::OPCODE_JUMP_IF_ARG_SUPPLIED => {
                let arg0 = match self.read_u8() {
                    Ok(b) => b,
                    Err(_) => {
                        return Err(DecodeError::InsufficientData);
                    }
                };
                let arg1 = match self.read_u16() {
                    Ok(w) => w,
                    Err(_) => {
                        return Err(DecodeError::InsufficientData);
                    }
                };
                Ok(Opcode::JumpIfArgSupplied(arg0, arg1))
            }
            haxby_opcodes::OPCODE_CALL => self
                .read_u8()
                .map_or(Err(DecodeError::InsufficientData), |b| Ok(Opcode::Call(b))),
            haxby_opcodes::OPCODE_RETURN => Ok(Opcode::Return),
            haxby_opcodes::OPCODE_GUARD_ENTER => Ok(Opcode::GuardEnter),
            haxby_opcodes::OPCODE_GUARD_EXIT => Ok(Opcode::GuardExit),
            haxby_opcodes::OPCODE_TRY_ENTER => self
                .read_u16()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::TryEnter(b))
                }),
            haxby_opcodes::OPCODE_TRY_EXIT => Ok(Opcode::TryExit),
            haxby_opcodes::OPCODE_THROW => Ok(Opcode::Throw),
            haxby_opcodes::OPCODE_BUILD_LIST => self
                .read_u32()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::BuildList(b))
                }),
            haxby_opcodes::OPCODE_BUILD_FUNCTION => self
                .read_u8()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::BuildFunction(b))
                }),
            haxby_opcodes::OPCODE_STORE_UPLEVEL => self
                .read_u8()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::StoreUplevel(b))
                }),
            haxby_opcodes::OPCODE_BUILD_STRUCT => Ok(Opcode::BuildStruct),
            haxby_opcodes::OPCODE_BUILD_ENUM => Ok(Opcode::BuildEnum),
            haxby_opcodes::OPCODE_BUILD_MIXIN => Ok(Opcode::BuildMixin),
            haxby_opcodes::OPCODE_BIND_METHOD => {
                let b0 = match self.read_u8() {
                    Ok(b) => b,
                    Err(_) => {
                        return Err(DecodeError::InsufficientData);
                    }
                };
                let w1 = match self.read_u16() {
                    Ok(w) => w,
                    Err(_) => {
                        return Err(DecodeError::InsufficientData);
                    }
                };
                Ok(Opcode::BindMethod(b0, w1))
            }
            haxby_opcodes::OPCODE_BIND_CASE => {
                let b0 = match self.read_u8() {
                    Ok(b) => b,
                    Err(_) => {
                        return Err(DecodeError::InsufficientData);
                    }
                };
                let w1 = match self.read_u16() {
                    Ok(w) => w,
                    Err(_) => {
                        return Err(DecodeError::InsufficientData);
                    }
                };
                Ok(Opcode::BindCase(b0, w1))
            }
            haxby_opcodes::OPCODE_INCLUDE_MIXIN => Ok(Opcode::IncludeMixin),
            haxby_opcodes::OPCODE_NEW_ENUM_VAL => self
                .read_u16()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::NewEnumVal(b))
                }),
            haxby_opcodes::OPCODE_ENUM_CHECK_IS_CASE => self
                .read_u16()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::EnumCheckIsCase(b))
                }),
            haxby_opcodes::OPCODE_ENUM_EXTRACT_PAYLOAD => Ok(Opcode::EnumExtractPayload),
            haxby_opcodes::OPCODE_TRY_UNWRAP_PROTOCOL => self
                .read_u8()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::TryUnwrapProtocol(b))
                }),
            haxby_opcodes::OPCODE_ISA => Ok(Opcode::Isa),
            haxby_opcodes::OPCODE_IMPORT => self
                .read_u16()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::Import(b))
                }),
            haxby_opcodes::OPCODE_LIFT_MODULE => Ok(Opcode::LiftModule),
            haxby_opcodes::OPCODE_LOAD_DYLIB => self
                .read_u16()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::LoadDylib(b))
                }),
            haxby_opcodes::OPCODE_ASSERT => self
                .read_u16()
                .map_or(Err(DecodeError::InsufficientData), |b| {
                    Ok(Opcode::Assert(b))
                }),
            haxby_opcodes::OPCODE_HALT => Ok(Opcode::Halt),
            _ => Err(DecodeError::UnknownOpcode(next)),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
