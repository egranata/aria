// SPDX-License-Identifier: Apache-2.0

use haxby_opcodes::function_attribs::{FUNC_IS_METHOD, METHOD_ATTRIBUTE_TYPE};
use haxby_vm::{
    builtins::VmBuiltins,
    error::{dylib_load::LoadResult, vm_error::VmErrorReason},
    frame::Frame,
    ok_or_err,
    runtime_module::RuntimeModule,
    runtime_value::{
        RuntimeValue, function::BuiltinFunctionImpl, list::List, object::Object,
        opaque::OpaqueValue,
    },
    some_or_err,
    vm::{self, RunloopExit},
};

use std::{cell::RefCell, path::PathBuf, time::SystemTime};

struct MutablePath {
    content: RefCell<std::path::PathBuf>,
}

#[derive(Default)]
struct New {}
impl BuiltinFunctionImpl for New {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let the_struct = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_struct().clone())?;
        let the_path =
            VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_string().cloned())?.raw_value();

        let pb = PathBuf::from(the_path);
        let pb = MutablePath {
            content: RefCell::new(pb),
        };

        let path_obj = OpaqueValue::new(pb);
        let aria_obj = Object::new(&the_struct);
        aria_obj.write("__path", RuntimeValue::Opaque(path_obj));
        frame.stack.push(RuntimeValue::Object(aria_obj));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD | METHOD_ATTRIBUTE_TYPE
    }

    fn required_argc(&self) -> u8 {
        2_u8
    }

    fn name(&self) -> &str {
        "_new"
    }
}

#[derive(Default)]
struct Cwd {}
impl BuiltinFunctionImpl for Cwd {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let the_struct = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_struct().clone())?;

        let cwd = ok_or_err!(
            std::env::current_dir(),
            VmErrorReason::UnexpectedVmState.into()
        );
        let cwd = MutablePath {
            content: RefCell::new(cwd),
        };

        let path_obj = OpaqueValue::new(cwd);
        let aria_obj = Object::new(&the_struct);
        aria_obj.write("__path", RuntimeValue::Opaque(path_obj));
        frame.stack.push(RuntimeValue::Object(aria_obj));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD | METHOD_ATTRIBUTE_TYPE
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "_cwd"
    }
}

#[derive(Default)]
struct Prettyprint {}
impl BuiltinFunctionImpl for Prettyprint {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let aria_object = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;

        let rust_obj = some_or_err!(
            aria_object.read("__path"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let rust_obj = some_or_err!(
            rust_obj.as_opaque_concrete::<MutablePath>(),
            VmErrorReason::UnexpectedVmState.into()
        );

        let rfo = rust_obj.content.borrow_mut();

        match rfo.as_os_str().to_str() {
            Some(s) => {
                frame.stack.push(RuntimeValue::String(s.into()));
                Ok(RunloopExit::Ok(()))
            }
            None => Err(VmErrorReason::UnexpectedVmState.into()),
        }
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "prettyprint"
    }
}

#[derive(Default)]
struct Append {}
impl BuiltinFunctionImpl for Append {
    fn eval(
        &self,
        frame: &mut Frame,
        vm: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let aria_object = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;
        let the_path =
            VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_string().cloned())?.raw_value();

        let rust_obj = some_or_err!(
            aria_object.read("__path"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let rust_obj = some_or_err!(
            rust_obj.as_opaque_concrete::<MutablePath>(),
            VmErrorReason::UnexpectedVmState.into()
        );

        let mut rfo = rust_obj.content.borrow_mut();
        rfo.push(the_path);

        frame.stack.push(ok_or_err!(
            vm.builtins.create_unit_object(),
            VmErrorReason::UnexpectedVmState.into()
        ));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        2_u8
    }

    fn name(&self) -> &str {
        "_append"
    }
}

#[derive(Default)]
struct Pop {}
impl BuiltinFunctionImpl for Pop {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let aria_object = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;

        let rust_obj = some_or_err!(
            aria_object.read("__path"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let rust_obj = some_or_err!(
            rust_obj.as_opaque_concrete::<MutablePath>(),
            VmErrorReason::UnexpectedVmState.into()
        );

        let mut rfo = rust_obj.content.borrow_mut();
        rfo.pop();
        frame.stack.push(RuntimeValue::Object(aria_object));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "pop"
    }
}

#[derive(Default)]
struct IsAbsolutePath {}
impl BuiltinFunctionImpl for IsAbsolutePath {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let aria_object = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;

        let rust_obj = some_or_err!(
            aria_object.read("__path"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let rust_obj = some_or_err!(
            rust_obj.as_opaque_concrete::<MutablePath>(),
            VmErrorReason::UnexpectedVmState.into()
        );

        let rfo = rust_obj.content.borrow_mut();
        frame
            .stack
            .push(RuntimeValue::Boolean((rfo.is_absolute()).into()));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "is_absolute"
    }
}

#[derive(Default)]
struct Exists {}
impl BuiltinFunctionImpl for Exists {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let aria_object = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;

        let rust_obj = some_or_err!(
            aria_object.read("__path"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let rust_obj = some_or_err!(
            rust_obj.as_opaque_concrete::<MutablePath>(),
            VmErrorReason::UnexpectedVmState.into()
        );

        let rfo = rust_obj.content.borrow_mut();
        frame
            .stack
            .push(RuntimeValue::Boolean((rfo.exists()).into()));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "exists"
    }
}

#[derive(Default)]
struct IsDirectory {}
impl BuiltinFunctionImpl for IsDirectory {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let aria_object = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;

        let rust_obj = some_or_err!(
            aria_object.read("__path"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let rust_obj = some_or_err!(
            rust_obj.as_opaque_concrete::<MutablePath>(),
            VmErrorReason::UnexpectedVmState.into()
        );

        let rfo = rust_obj.content.borrow_mut();
        frame
            .stack
            .push(RuntimeValue::Boolean((rfo.is_dir()).into()));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "is_directory"
    }
}

#[derive(Default)]
struct IsFile {}
impl BuiltinFunctionImpl for IsFile {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let aria_object = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;

        let rust_obj = some_or_err!(
            aria_object.read("__path"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let rust_obj = some_or_err!(
            rust_obj.as_opaque_concrete::<MutablePath>(),
            VmErrorReason::UnexpectedVmState.into()
        );

        let rfo = rust_obj.content.borrow_mut();
        frame
            .stack
            .push(RuntimeValue::Boolean((rfo.is_file()).into()));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "is_file"
    }
}

#[derive(Default)]
struct IsSymlink {}
impl BuiltinFunctionImpl for IsSymlink {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let aria_object = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;

        let rust_obj = some_or_err!(
            aria_object.read("__path"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let rust_obj = some_or_err!(
            rust_obj.as_opaque_concrete::<MutablePath>(),
            VmErrorReason::UnexpectedVmState.into()
        );

        let rfo = rust_obj.content.borrow_mut();
        frame
            .stack
            .push(RuntimeValue::Boolean((rfo.is_symlink()).into()));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "is_symlink"
    }
}

#[derive(Default)]
struct Canonical {}
impl BuiltinFunctionImpl for Canonical {
    fn eval(
        &self,
        frame: &mut Frame,
        vm: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let aria_object = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;

        let rust_obj = some_or_err!(
            aria_object.read("__path"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let rust_obj = some_or_err!(
            rust_obj.as_opaque_concrete::<MutablePath>(),
            VmErrorReason::UnexpectedVmState.into()
        );

        let rfo = rust_obj.content.borrow_mut();
        let canonical_rfo = match rfo.canonicalize() {
            Ok(path) => path,
            Err(_) => {
                let none = ok_or_err!(
                    vm.builtins.create_maybe_none(),
                    VmErrorReason::UnexpectedVmState.into()
                );
                frame.stack.push(none);
                return Ok(RunloopExit::Ok(()));
            }
        };

        let canonical_object = Object::new(aria_object.get_struct());
        let canonical_rfo = MutablePath {
            content: RefCell::new(canonical_rfo),
        };

        let canonical_path_obj = OpaqueValue::new(canonical_rfo);
        canonical_object.write("__path", RuntimeValue::Opaque(canonical_path_obj));

        let canonical_object = RuntimeValue::Object(canonical_object);
        let some = ok_or_err!(
            vm.builtins.create_maybe_some(canonical_object),
            VmErrorReason::UnexpectedVmState.into()
        );

        frame.stack.push(some);
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "new_canonical"
    }
}

#[derive(Default)]
struct Size {}
impl BuiltinFunctionImpl for Size {
    fn eval(
        &self,
        frame: &mut Frame,
        vm: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let aria_object = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;

        let rust_obj = some_or_err!(
            aria_object.read("__path"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let rust_obj = some_or_err!(
            rust_obj.as_opaque_concrete::<MutablePath>(),
            VmErrorReason::UnexpectedVmState.into()
        );

        let rfo = rust_obj.content.borrow_mut();
        match rfo.metadata() {
            Ok(md) => {
                frame
                    .stack
                    .push(RuntimeValue::Integer((md.len() as i64).into()));
            }
            Err(_) => {
                let val = ok_or_err!(
                    vm.builtins.create_maybe_none(),
                    VmErrorReason::UnexpectedVmState.into()
                );
                frame.stack.push(val);
            }
        }
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "size"
    }
}

#[derive(Default)]
struct CreatedTime {}
impl BuiltinFunctionImpl for CreatedTime {
    fn eval(
        &self,
        frame: &mut Frame,
        vm: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let aria_object = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;

        let rust_obj = some_or_err!(
            aria_object.read("__path"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let rust_obj = some_or_err!(
            rust_obj.as_opaque_concrete::<MutablePath>(),
            VmErrorReason::UnexpectedVmState.into()
        );

        let rfo = rust_obj.content.borrow_mut();
        match rfo.metadata() {
            Ok(md) => match md.created() {
                Err(_) => {
                    let val = ok_or_err!(
                        vm.builtins.create_maybe_none(),
                        VmErrorReason::UnexpectedVmState.into()
                    );
                    frame.stack.push(val);
                }
                Ok(val) => {
                    let val = val
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis();
                    frame.stack.push(RuntimeValue::Integer((val as i64).into()));
                }
            },
            Err(_) => {
                let val = ok_or_err!(
                    vm.builtins.create_maybe_none(),
                    VmErrorReason::UnexpectedVmState.into()
                );
                frame.stack.push(val);
            }
        }
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "_when_created"
    }
}

#[derive(Default)]
struct AccessedTime {}
impl BuiltinFunctionImpl for AccessedTime {
    fn eval(
        &self,
        frame: &mut Frame,
        vm: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let aria_object = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;

        let rust_obj = some_or_err!(
            aria_object.read("__path"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let rust_obj = some_or_err!(
            rust_obj.as_opaque_concrete::<MutablePath>(),
            VmErrorReason::UnexpectedVmState.into()
        );

        let rfo = rust_obj.content.borrow_mut();
        match rfo.metadata() {
            Ok(md) => {
                match md.accessed() {
                    Err(_) => {
                        let val = ok_or_err!(
                            vm.builtins.create_maybe_none(),
                            VmErrorReason::UnexpectedVmState.into()
                        );
                        frame.stack.push(val);
                    }
                    Ok(val) => {
                        let val = val
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_millis();
                        frame.stack.push(RuntimeValue::Integer((val as i64).into()));
                    }
                }
                frame
                    .stack
                    .push(RuntimeValue::Integer((md.len() as i64).into()));
            }
            Err(_) => {
                let val = ok_or_err!(
                    vm.builtins.create_maybe_none(),
                    VmErrorReason::UnexpectedVmState.into()
                );
                frame.stack.push(val);
            }
        }
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "_when_accessed"
    }
}

#[derive(Default)]
struct Filename {}
impl BuiltinFunctionImpl for Filename {
    fn eval(
        &self,
        frame: &mut Frame,
        vm: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let aria_object = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;

        let rust_obj = some_or_err!(
            aria_object.read("__path"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let rust_obj = some_or_err!(
            rust_obj.as_opaque_concrete::<MutablePath>(),
            VmErrorReason::UnexpectedVmState.into()
        );

        let rfo = rust_obj.content.borrow_mut();
        match rfo.file_name() {
            Some(name) => {
                let name = some_or_err!(name.to_str(), VmErrorReason::UnexpectedVmState.into());
                frame.stack.push(RuntimeValue::String(name.into()));
            }
            None => {
                let val = ok_or_err!(
                    vm.builtins.create_maybe_none(),
                    VmErrorReason::UnexpectedVmState.into()
                );
                frame.stack.push(val);
            }
        }
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "get_filename"
    }
}

#[derive(Default)]
struct Extension {}
impl BuiltinFunctionImpl for Extension {
    fn eval(
        &self,
        frame: &mut Frame,
        vm: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let aria_object = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;

        let rust_obj = some_or_err!(
            aria_object.read("__path"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let rust_obj = some_or_err!(
            rust_obj.as_opaque_concrete::<MutablePath>(),
            VmErrorReason::UnexpectedVmState.into()
        );

        let rfo = rust_obj.content.borrow_mut();
        match rfo.extension() {
            Some(name) => {
                let name = some_or_err!(name.to_str(), VmErrorReason::UnexpectedVmState.into());
                frame.stack.push(RuntimeValue::String(name.into()));
            }
            None => {
                let val = ok_or_err!(
                    vm.builtins.create_maybe_none(),
                    VmErrorReason::UnexpectedVmState.into()
                );
                frame.stack.push(val);
            }
        }
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "get_extension"
    }
}

#[derive(Default)]
struct Entries {}
impl BuiltinFunctionImpl for Entries {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let aria_object = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;

        let rust_obj = some_or_err!(
            aria_object.read("__path"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let rust_obj = some_or_err!(
            rust_obj.as_opaque_concrete::<MutablePath>(),
            VmErrorReason::UnexpectedVmState.into()
        );

        let aria_struct = aria_object.get_struct();

        let rfo = rust_obj.content.borrow_mut();
        let list = List::from(&[]);
        if let Ok(rd) = rfo.read_dir() {
            for entry in rd.flatten() {
                let entry_object = Object::new(aria_struct);
                let entry_refcell = MutablePath {
                    content: RefCell::new(entry.path()),
                };

                let entry_opaque = OpaqueValue::new(entry_refcell);
                entry_object.write("__path", RuntimeValue::Opaque(entry_opaque));
                list.append(RuntimeValue::Object(entry_object));
            }
        }

        frame.stack.push(RuntimeValue::List(list));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "entries"
    }
}

#[derive(Default)]
struct MakeDirectory {}
impl BuiltinFunctionImpl for MakeDirectory {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let aria_object = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;

        let rust_obj = some_or_err!(
            aria_object.read("__path"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let rust_obj = some_or_err!(
            rust_obj.as_opaque_concrete::<MutablePath>(),
            VmErrorReason::UnexpectedVmState.into()
        );

        let rfo = rust_obj.content.borrow_mut();
        frame.stack.push(RuntimeValue::Boolean(
            std::fs::create_dir(rfo.as_path()).is_ok().into(),
        ));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "mkdir"
    }
}

#[derive(Default)]
struct RemoveDirectory {}
impl BuiltinFunctionImpl for RemoveDirectory {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let aria_object = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;

        let rust_obj = some_or_err!(
            aria_object.read("__path"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let rust_obj = some_or_err!(
            rust_obj.as_opaque_concrete::<MutablePath>(),
            VmErrorReason::UnexpectedVmState.into()
        );

        let rfo = rust_obj.content.borrow_mut();
        frame.stack.push(RuntimeValue::Boolean(
            std::fs::remove_dir(rfo.as_path()).is_ok().into(),
        ));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "rmdir"
    }
}

#[derive(Default)]
struct RemoveFile {}
impl BuiltinFunctionImpl for RemoveFile {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let aria_object = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;

        let rust_obj = some_or_err!(
            aria_object.read("__path"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let rust_obj = some_or_err!(
            rust_obj.as_opaque_concrete::<MutablePath>(),
            VmErrorReason::UnexpectedVmState.into()
        );

        let rfo = rust_obj.content.borrow_mut();
        frame.stack.push(RuntimeValue::Boolean(
            std::fs::remove_file(rfo.as_path()).is_ok().into(),
        ));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        1_u8
    }

    fn name(&self) -> &str {
        "erase"
    }
}

#[derive(Default)]
struct Copy {}
impl BuiltinFunctionImpl for Copy {
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut vm::VirtualMachine,
    ) -> vm::ExecutionResult<RunloopExit> {
        let this_path = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;
        let other_path = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_object().cloned())?;

        let this_path = some_or_err!(
            this_path.read("__path"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let other_path = some_or_err!(
            other_path.read("__path"),
            VmErrorReason::UnexpectedVmState.into()
        );

        let this_path = some_or_err!(
            this_path.as_opaque_concrete::<MutablePath>(),
            VmErrorReason::UnexpectedVmState.into()
        );
        let other_path = some_or_err!(
            other_path.as_opaque_concrete::<MutablePath>(),
            VmErrorReason::UnexpectedVmState.into()
        );

        let this_path = this_path.content.borrow_mut();
        let other_path = other_path.content.borrow_mut();

        frame.stack.push(RuntimeValue::Boolean(
            std::fs::copy(this_path.as_path(), other_path.as_path())
                .is_ok()
                .into(),
        ));
        Ok(RunloopExit::Ok(()))
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD
    }

    fn required_argc(&self) -> u8 {
        2_u8
    }

    fn name(&self) -> &str {
        "_copy"
    }
}

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn dylib_haxby_inject(module: *const RuntimeModule) -> LoadResult {
    match unsafe { module.as_ref() } {
        Some(module) => {
            let path = match module.load_named_value("Path") {
                Some(path) => path,
                None => {
                    return LoadResult::error("cannot find Path");
                }
            };

            let path_struct = match path.as_struct() {
                Some(path) => path,
                None => {
                    return LoadResult::error("Path is not a struct");
                }
            };

            path_struct.insert_builtin::<New>();
            path_struct.insert_builtin::<Cwd>();
            path_struct.insert_builtin::<Prettyprint>();
            path_struct.insert_builtin::<Append>();
            path_struct.insert_builtin::<Pop>();
            path_struct.insert_builtin::<IsAbsolutePath>();
            path_struct.insert_builtin::<Exists>();
            path_struct.insert_builtin::<IsDirectory>();
            path_struct.insert_builtin::<IsSymlink>();
            path_struct.insert_builtin::<IsFile>();
            path_struct.insert_builtin::<Canonical>();
            path_struct.insert_builtin::<Size>();
            path_struct.insert_builtin::<Entries>();
            path_struct.insert_builtin::<Filename>();
            path_struct.insert_builtin::<Extension>();
            path_struct.insert_builtin::<CreatedTime>();
            path_struct.insert_builtin::<AccessedTime>();
            path_struct.insert_builtin::<MakeDirectory>();
            path_struct.insert_builtin::<RemoveDirectory>();
            path_struct.insert_builtin::<RemoveFile>();
            path_struct.insert_builtin::<Copy>();

            LoadResult::success()
        }
        None => LoadResult::error("invalid path module"),
    }
}
