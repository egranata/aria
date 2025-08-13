// SPDX-License-Identifier: Apache-2.0
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use aria_compiler::line_table::LineTable;
use aria_parser::ast::SourcePointer;
use haxby_opcodes::function_attribs::{FUNC_ACCEPTS_VARARG, FUNC_IS_METHOD, METHOD_ATTRIBUTE_TYPE};

use crate::{
    frame::Frame,
    runtime_module::RuntimeModule,
    vm::{ExecutionResult, RunloopExit, VirtualMachine},
};

use super::{
    CallResult, RuntimeValue, list::List, object::ObjectBox, runtime_code_object::CodeObject,
};

pub trait BuiltinFunctionImpl {
    fn eval(&self, frame: &mut Frame, vm: &mut VirtualMachine) -> ExecutionResult<RunloopExit>;
    fn required_argc(&self) -> u8;
    fn default_argc(&self) -> u8 {
        0
    }
    fn attrib_byte(&self) -> u8 {
        0
    }
    fn name(&self) -> &str;
}

pub struct BuiltinFunction {
    pub body: Rc<dyn BuiltinFunctionImpl>,
    pub(crate) boxx: ObjectBox,
}

impl BuiltinFunction {
    pub fn new(body: Rc<dyn BuiltinFunctionImpl>) -> Self {
        Self {
            body,
            boxx: Default::default(),
        }
    }
}

pub struct BytecodeFunction {
    pub name: String,
    pub body: Rc<[u8]>,
    pub required_argc: u8,
    pub default_argc: u8,
    pub frame_size: u8,
    pub line_table: Rc<LineTable>,
    pub loc: SourcePointer,
    pub attrib_byte: u8,
    pub module: RuntimeModule,
    pub(crate) boxx: ObjectBox,
    uplevels: std::cell::RefCell<HashMap<u8, RuntimeValue>>,
}

impl BytecodeFunction {
    pub(crate) fn store_uplevel(&self, idx: u8, val: RuntimeValue) {
        self.uplevels.borrow_mut().insert(idx, val);
    }

    pub(crate) fn read_uplevel(&self, idx: u8) -> Option<RuntimeValue> {
        self.uplevels.borrow().get(&idx).cloned()
    }
}

#[derive(enum_as_inner::EnumAsInner)]
pub(crate) enum FunctionImpl {
    BytecodeFunction(BytecodeFunction),
    BuiltinFunction(BuiltinFunction),
}

#[derive(Clone)]
pub struct Function {
    pub(crate) imp: Rc<FunctionImpl>,
}

impl FunctionImpl {
    pub(crate) fn attribute(&self) -> FunctionAttribute {
        match self {
            Self::BytecodeFunction(bc) => FunctionAttribute::from(bc.attrib_byte),
            Self::BuiltinFunction(bf) => FunctionAttribute::from(bf.body.attrib_byte()),
        }
    }

    pub(crate) fn line_table(&self) -> Option<&LineTable> {
        match self {
            Self::BytecodeFunction(bc) => Some(&bc.line_table),
            Self::BuiltinFunction(_) => None,
        }
    }

    pub(crate) fn required_argc(&self) -> u8 {
        match self {
            Self::BytecodeFunction(bc) => bc.required_argc,
            Self::BuiltinFunction(bf) => bf.body.required_argc(),
        }
    }

    pub(crate) fn default_argc(&self) -> u8 {
        match self {
            Self::BytecodeFunction(bc) => bc.default_argc,
            Self::BuiltinFunction(bf) => bf.body.default_argc(),
        }
    }

    pub(crate) fn frame_size(&self) -> u8 {
        match self {
            Self::BytecodeFunction(bc) => bc.frame_size,
            Self::BuiltinFunction(_) => 0,
        }
    }

    pub(crate) fn name(&self) -> &str {
        match self {
            Self::BytecodeFunction(bc) => &bc.name,
            Self::BuiltinFunction(bf) => bf.body.name(),
        }
    }

    pub(crate) fn loc(&self) -> Option<&SourcePointer> {
        match self {
            Self::BytecodeFunction(bc) => Some(&bc.loc),
            Self::BuiltinFunction(_) => None,
        }
    }
}

impl Function {
    pub fn attribute(&self) -> FunctionAttribute {
        self.imp.attribute()
    }

    pub fn line_table(&self) -> Option<&LineTable> {
        self.imp.line_table()
    }

    pub fn required_argc(&self) -> u8 {
        self.imp.required_argc()
    }

    pub fn default_argc(&self) -> u8 {
        self.imp.default_argc()
    }

    pub fn frame_size(&self) -> u8 {
        self.imp.frame_size()
    }

    pub fn varargs(&self) -> bool {
        self.attribute().is_vararg()
    }

    pub fn name(&self) -> &str {
        self.imp.name()
    }

    pub fn loc(&self) -> Option<&SourcePointer> {
        self.imp.loc()
    }

    pub fn identity(&self) -> usize {
        Rc::as_ptr(&self.imp) as usize
    }
}

pub struct FunctionAttribute {
    val: u8,
}

impl From<u8> for FunctionAttribute {
    fn from(val: u8) -> Self {
        Self { val }
    }
}

impl FunctionAttribute {
    pub fn is_free(&self) -> bool {
        self.val & FUNC_IS_METHOD == 0
    }

    pub fn is_vararg(&self) -> bool {
        self.val & FUNC_ACCEPTS_VARARG != 0
    }

    pub fn is_method(&self) -> bool {
        self.val & FUNC_IS_METHOD == FUNC_IS_METHOD
    }

    pub fn is_instance_method(&self) -> bool {
        self.is_method() && (self.val & METHOD_ATTRIBUTE_TYPE == 0)
    }

    pub fn is_type_method(&self) -> bool {
        self.is_method() && (self.val & METHOD_ATTRIBUTE_TYPE == METHOD_ATTRIBUTE_TYPE)
    }
}

impl FunctionImpl {
    pub fn new_builtin<T>() -> Self
    where
        T: 'static + BuiltinFunctionImpl + Default,
    {
        Self::BuiltinFunction(BuiltinFunction::new(Rc::new(T::default())))
    }

    pub fn builtin_from<T>(val: T) -> Self
    where
        T: 'static + BuiltinFunctionImpl + Default,
    {
        Self::BuiltinFunction(BuiltinFunction::new(Rc::new(val)))
    }

    pub fn from_code_object(co: &CodeObject, a: u8, m: &RuntimeModule) -> Self {
        let rc = co.body.clone();
        let lt = co.line_table.clone();
        let bcf = BytecodeFunction {
            name: co.name.clone(),
            body: rc,
            required_argc: co.required_argc,
            default_argc: co.default_argc,
            frame_size: co.frame_size,
            line_table: lt,
            loc: co.loc.clone(),
            attrib_byte: a,
            module: m.clone(),
            boxx: Default::default(),
            uplevels: Default::default(),
        };
        Self::BytecodeFunction(bcf)
    }

    fn write(&self, name: &str, val: RuntimeValue) {
        match self {
            FunctionImpl::BytecodeFunction(b) => &b.boxx,
            FunctionImpl::BuiltinFunction(b) => &b.boxx,
        }
        .write(name, val)
    }

    fn read(&self, name: &str) -> Option<RuntimeValue> {
        match self {
            FunctionImpl::BytecodeFunction(b) => &b.boxx,
            FunctionImpl::BuiltinFunction(b) => &b.boxx,
        }
        .read(name)
    }

    fn list_attributes(&self) -> HashSet<String> {
        match self {
            FunctionImpl::BytecodeFunction(b) => b.boxx.list_attributes(),
            FunctionImpl::BuiltinFunction(b) => b.boxx.list_attributes(),
        }
    }
}

impl Function {
    pub fn new_builtin<T>() -> Self
    where
        T: 'static + BuiltinFunctionImpl + Default,
    {
        Self {
            imp: Rc::new(FunctionImpl::new_builtin::<T>()),
        }
    }

    pub fn builtin_from<T>(val: T) -> Self
    where
        T: 'static + BuiltinFunctionImpl + Default,
    {
        Self {
            imp: Rc::new(FunctionImpl::builtin_from(val)),
        }
    }

    pub fn from_code_object(co: &CodeObject, a: u8, m: &RuntimeModule) -> Self {
        Self {
            imp: Rc::new(FunctionImpl::from_code_object(co, a, m)),
        }
    }

    // DO NOT CALL unless you are Function or BoundFunction
    pub(super) fn eval_in_frame(
        &self,
        argc: u8,
        target_frame: &mut Frame,
        vm: &mut VirtualMachine,
    ) -> ExecutionResult<RunloopExit> {
        match self.imp.as_ref() {
            FunctionImpl::BytecodeFunction(bcf) => {
                target_frame.set_argc(argc);
                vm.eval_bytecode_in_frame(&bcf.module, &bcf.body, target_frame)
            }
            FunctionImpl::BuiltinFunction(bnf) => bnf.body.eval(target_frame, vm),
        }
    }

    pub fn eval(
        &self,
        argc: u8,
        cur_frame: &mut Frame,
        vm: &mut VirtualMachine,
        discard_result: bool,
    ) -> ExecutionResult<CallResult> {
        let mut new_frame = Frame::new_with_function(self.clone());

        if self.attribute().is_vararg() {
            if argc < self.required_argc() {
                return Err(
                    crate::error::vm_error::VmErrorReason::MismatchedArgumentCount(
                        self.required_argc() as usize,
                        argc as usize,
                    )
                    .into(),
                );
            }

            let l = List::default();
            for i in 0..argc {
                if i < self.required_argc() {
                    new_frame.stack.at_head(cur_frame.stack.pop());
                } else {
                    l.append(cur_frame.stack.pop());
                }
            }

            new_frame.stack.at_head(super::RuntimeValue::List(l));
        } else {
            if argc != self.required_argc() {
                return Err(
                    crate::error::vm_error::VmErrorReason::MismatchedArgumentCount(
                        self.required_argc() as usize,
                        argc as usize,
                    )
                    .into(),
                );
            }

            for _ in 0..argc {
                new_frame.stack.at_head(cur_frame.stack.pop());
            }
        }

        match self.eval_in_frame(argc, &mut new_frame, vm)? {
            RunloopExit::Ok(_) => match new_frame.stack.try_pop() {
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

    pub fn write(&self, name: &str, val: RuntimeValue) {
        self.imp.write(name, val)
    }

    pub fn read(&self, name: &str) -> Option<RuntimeValue> {
        self.imp.read(name)
    }

    pub fn list_attributes(&self) -> HashSet<String> {
        self.imp.list_attributes()
    }
}

impl PartialEq for FunctionImpl {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::BytecodeFunction(l0), Self::BytecodeFunction(r0)) => {
                Rc::ptr_eq(&l0.body, &r0.body)
            }
            (Self::BuiltinFunction(l0), Self::BuiltinFunction(r0)) => {
                Rc::ptr_eq(&l0.body, &r0.body)
            }
            _ => false,
        }
    }
}
impl Eq for FunctionImpl {}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.imp, &other.imp) || self.imp.eq(&other.imp)
    }
}
impl Eq for Function {}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name();
        if let Some(loc) = self.loc() {
            write!(f, "<function {name} at {loc}>")
        } else {
            write!(f, "<function {name}>")
        }
    }
}
