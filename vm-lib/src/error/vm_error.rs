// SPDX-License-Identifier: Apache-2.0
use aria_parser::ast::{SourcePointer, prettyprint::printout_accumulator::PrintoutAccumulator};
use haxby_opcodes::Opcode;
use thiserror::Error;

use crate::{opcodes::prettyprint::opcode_prettyprint, runtime_module::RuntimeModule};

#[derive(Error, PartialEq, Eq, Debug)]
pub enum VmErrorReason {
    #[error("assertion failed: {0}")]
    AssertFailed(String),

    #[error("'{0}' is a circular import reference")]
    CircularImport(String),

    #[error("division by zero")]
    DivisionByZero,

    #[error("enum value has no payload")]
    EnumWithoutPayload,

    #[error("runtime stack is empty")]
    EmptyStack,

    #[error("index {0} out of bounds")]
    IndexOutOfBounds(usize),

    #[error("cannot import module at path '{0}': {1}")]
    ImportNotAvailable(String, String),

    #[error("instruction cannot be fully decoded")]
    IncompleteInstruction,

    #[error("invalid binding")]
    InvalidBinding,

    #[error("control instruction invalid")]
    InvalidControlInstruction,

    #[error("mismatched argument count, expected {0} actual {1}")]
    MismatchedArgumentCount(usize, usize),

    #[error("unknown named identifier: '{0}'")]
    NoSuchIdentifier(String),

    #[error("'{0}' is not a valid case for this enum")]
    NoSuchCase(String),

    #[error("operation failed: {0}")]
    OperationFailed(String),

    #[error("unexpected value type")]
    UnexpectedType,

    #[error("VM execution is not a valid state")]
    UnexpectedVmState,

    #[error("uplevel {0} not available")]
    UplevelOutOfBounds(usize),

    #[error("{0} is not a known opcode")]
    UnknownOpcode(u8),

    #[error("VM execution halted")]
    VmHalted,
}

pub struct VmError {
    pub reason: VmErrorReason,
    pub opcode: Option<Opcode>,
    pub loc: Option<SourcePointer>,
}

impl VmError {
    pub fn prettyprint(&self, module: Option<RuntimeModule>) -> String {
        let mut poa = PrintoutAccumulator::default();
        poa = poa << "vm error: " << self.reason.to_string();
        if let Some(opcode) = &self.opcode {
            if let Some(m) = module {
                poa = opcode_prettyprint(opcode, &m, poa << " opcode: ");
            }
        }
        if let Some(loc) = &self.loc {
            poa = poa << " at " << loc.to_string();
        }

        poa.value()
    }
}

impl std::fmt::Debug for VmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.prettyprint(None))
    }
}

impl From<VmErrorReason> for VmError {
    fn from(reason: VmErrorReason) -> Self {
        Self {
            reason,
            opcode: None,
            loc: None,
        }
    }
}
