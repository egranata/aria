// SPDX-License-Identifier: Apache-2.0
use std::rc::Rc;

use crate::{
    frame::Frame,
    vm::{ExecutionResult, RunloopExit, VirtualMachine},
};

use super::{CallResult, RuntimeValue, function::Function, list::List};

struct BoundFunctionImpl {
    this: RuntimeValue,
    func: Function,
}

#[derive(Clone)]
pub struct BoundFunction {
    imp: Rc<BoundFunctionImpl>,
}

impl BoundFunction {
    pub(super) fn bind(this: RuntimeValue, func: Function) -> Self {
        Self {
            imp: Rc::new(BoundFunctionImpl { this, func }),
        }
    }

    pub fn this(&self) -> &RuntimeValue {
        &self.imp.this
    }

    pub fn func(&self) -> &Function {
        &self.imp.func
    }

    pub fn eval(
        &self,
        argc: u8,
        cur_frame: &mut Frame,
        vm: &mut VirtualMachine,
        discard_result: bool,
    ) -> ExecutionResult<CallResult> {
        let mut new_frame = Frame::new_with_function(self.func().clone());

        if self.func().attribute().is_vararg() {
            if 1 + argc < self.func().arity().required {
                return Err(
                    crate::error::vm_error::VmErrorReason::MismatchedArgumentCount(
                        self.func().arity().required as usize,
                        argc as usize,
                    )
                    .into(),
                );
            }

            let l = List::default();
            for i in 0..argc {
                let arg = cur_frame.stack.pop();
                if i < self.func().arity().required - 1 {
                    new_frame.stack.at_head(arg);
                } else {
                    l.append(arg);
                }
            }

            new_frame.stack.at_head(super::RuntimeValue::List(l));
        } else {
            if 1 + argc != self.func().arity().required {
                return Err(
                    crate::error::vm_error::VmErrorReason::MismatchedArgumentCount(
                        self.func().arity().required as usize,
                        argc as usize,
                    )
                    .into(),
                );
            }

            for _ in 0..argc {
                new_frame.stack.at_head(cur_frame.stack.pop());
            }
        }

        new_frame.stack.push(self.this().clone());

        match self.imp.func.eval_in_frame(argc, &mut new_frame, vm)? {
            RunloopExit::Ok(()) => match new_frame.stack.try_pop() {
                Some(ret) => {
                    if !discard_result {
                        cur_frame.stack.push(ret.clone());
                    }
                    Ok(CallResult::Ok(ret))
                }
                _ => Ok(CallResult::OkNoValue),
            },
            RunloopExit::Exception(e) => Ok(CallResult::Exception(e)),
        }
    }

    pub fn identity(&self) -> usize {
        Rc::as_ptr(&self.imp) as usize
    }
}

impl PartialEq for BoundFunction {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.imp, &other.imp)
    }
}
impl Eq for BoundFunction {}
