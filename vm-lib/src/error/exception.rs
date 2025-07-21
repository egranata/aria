// SPDX-License-Identifier: Apache-2.0

use aria_parser::ast::SourcePointer;
use haxby_opcodes::builtin_type_ids::BUILTIN_TYPE_RUNTIME_ERROR;

use crate::{
    builtins::VmBuiltins,
    error::vm_error::{VmError, VmErrorReason},
    runtime_value::{object::Object, RuntimeValue},
    vm::VirtualMachine,
};

#[derive(Clone, Default)]
pub struct Backtrace {
    entries: Vec<SourcePointer>,
}

impl Backtrace {
    pub fn first_entry(&self) -> Option<SourcePointer> {
        self.entries.first().cloned()
    }

    pub fn entries_iter(&self) -> std::slice::Iter<'_, SourcePointer> {
        self.entries.iter()
    }

    fn len(&self) -> usize {
        self.entries.len()
    }
}

pub struct VmException {
    pub value: RuntimeValue,
    pub backtrace: Backtrace,
}

impl VmException {
    pub fn from_value(value: RuntimeValue) -> Self {
        Self {
            value,
            backtrace: Default::default(),
        }
    }

    pub fn from_value_and_loc(value: RuntimeValue, loc: Option<SourcePointer>) -> Self {
        let mut this = VmException::from_value(value);
        if let Some(loc) = loc {
            this = this.thrown_at(loc);
        }

        this
    }

    pub fn thrown_at(self, loc: SourcePointer) -> Self {
        if self.backtrace.len() == 1 && self.backtrace.first_entry().unwrap() == loc {
            self
        } else {
            let mut new_bt = self.backtrace.clone();
            new_bt.entries.push(loc);
            Self {
                value: self.value.clone(),
                backtrace: new_bt,
            }
        }
    }

    pub fn is_builtin_unimplemented(&self, vm: &mut VirtualMachine) -> bool {
        self.value.is_builtin_unimplemented(vm)
    }
}

#[macro_export]
macro_rules! some_or_err {
    ($opt:expr, $err:expr) => {
        match $opt {
            Some(val) => val,
            None => return Err($err),
        }
    };
}

#[macro_export]
macro_rules! ok_or_err {
    ($opt:expr, $err:expr) => {
        match $opt {
            Ok(val) => val,
            Err(_) => return Err($err),
        }
    };
}

impl VmException {
    pub fn from_vmerror(err: VmError, builtins: &VmBuiltins) -> Result<VmException, VmError> {
        let rt_err_type = some_or_err!(
            builtins.get_builtin_type_by_id(BUILTIN_TYPE_RUNTIME_ERROR),
            err
        );

        let rt_err = some_or_err!(rt_err_type.as_enum(), err);

        struct ExceptionData {
            case: usize,
            payload: Option<RuntimeValue>,
        }

        let e_data = match &err.reason {
            VmErrorReason::DivisionByZero => ExceptionData {
                case: some_or_err!(rt_err.get_idx_of_case("DivisionByZero"), err),
                payload: None,
            },
            VmErrorReason::EnumWithoutPayload => ExceptionData {
                case: some_or_err!(rt_err.get_idx_of_case("EnumWithoutPayload"), err),
                payload: None,
            },
            VmErrorReason::IndexOutOfBounds(idx) => ExceptionData {
                case: some_or_err!(rt_err.get_idx_of_case("IndexOutOfBounds"), err),
                payload: Some(RuntimeValue::Integer((*idx as i64).into())),
            },
            VmErrorReason::MismatchedArgumentCount(expected, actual) => {
                let argc_mismatch = some_or_err!(rt_err.load_named_value("ArgcMismatch"), err);
                let argc_mismatch = some_or_err!(argc_mismatch.as_struct(), err);
                let argc_mismatch_obj = Object::new(&argc_mismatch);
                argc_mismatch_obj
                    .write("expected", RuntimeValue::Integer((*expected as i64).into()));
                argc_mismatch_obj.write("actual", RuntimeValue::Integer((*actual as i64).into()));
                ExceptionData {
                    case: some_or_err!(rt_err.get_idx_of_case("MismatchedArgumentCount"), err),
                    payload: Some(RuntimeValue::Object(argc_mismatch_obj)),
                }
            }
            VmErrorReason::NoSuchCase(s) => ExceptionData {
                case: some_or_err!(rt_err.get_idx_of_case("NoSuchCase"), err),
                payload: Some(RuntimeValue::String(s.clone().into())),
            },
            VmErrorReason::NoSuchIdentifier(s) => ExceptionData {
                case: some_or_err!(rt_err.get_idx_of_case("NoSuchIdentifier"), err),
                payload: Some(RuntimeValue::String(s.clone().into())),
            },
            VmErrorReason::UnexpectedType => ExceptionData {
                case: some_or_err!(rt_err.get_idx_of_case("UnexpectedType"), err),
                payload: None,
            },
            _ => {
                return Err(err);
            }
        };

        let exception_value = RuntimeValue::EnumValue(some_or_err!(
            rt_err.make_value(e_data.case, e_data.payload),
            err
        ));
        Ok(VmException::from_value_and_loc(exception_value, err.loc))
    }
}
