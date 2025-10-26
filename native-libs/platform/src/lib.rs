// SPDX-License-Identifier: Apache-2.0

use haxby_opcodes::function_attribs::{FUNC_IS_METHOD, METHOD_ATTRIBUTE_TYPE};
use haxby_vm::{
    builtins::VmBuiltins, error::dylib_load::LoadResult, frame::Frame,
    runtime_module::RuntimeModule, runtime_value::RuntimeValue,
    runtime_value::function::BuiltinFunctionImpl, vm::RunloopExit,
};

#[derive(Default)]
struct GetPlatformInfo {}
impl BuiltinFunctionImpl for GetPlatformInfo {
    #[cfg(target_os = "linux")]
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut haxby_vm::vm::VirtualMachine,
    ) -> haxby_vm::vm::ExecutionResult<RunloopExit> {
        use haxby_vm::{
            error::vm_error::VmErrorReason, runtime_value::object::Object, some_or_err,
        };

        let kernel_version = match std::fs::read_to_string("/proc/sys/kernel/osrelease") {
            Ok(ver) => ver.trim().to_string(),
            Err(_) => String::from("unknown"),
        };

        let platform_enum = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_enum().cloned())?;

        let linux_info = some_or_err!(
            platform_enum.load_named_value("LinuxPlatform"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let linux_info = some_or_err!(
            linux_info.as_struct(),
            VmErrorReason::UnexpectedVmState.into()
        );
        let linux_info = Object::new(linux_info);
        linux_info.write(
            "kernel_version",
            RuntimeValue::String(kernel_version.into()),
        );

        let linux_case = some_or_err!(
            platform_enum.get_idx_of_case("Linux"),
            VmErrorReason::UnexpectedVmState.into()
        );

        let linux_enum_instance = some_or_err!(
            platform_enum.make_value(linux_case, Some(RuntimeValue::Object(linux_info))),
            VmErrorReason::UnexpectedVmState.into()
        );

        frame
            .stack
            .push(RuntimeValue::EnumValue(linux_enum_instance));
        Ok(RunloopExit::Ok(()))
    }

    #[cfg(target_os = "macos")]
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut haxby_vm::vm::VirtualMachine,
    ) -> haxby_vm::vm::ExecutionResult<RunloopExit> {
        use haxby_vm::{
            error::vm_error::VmErrorReason, runtime_value::object::Object, some_or_err,
        };

        // Get macOS version using `sw_vers -productVersion`
        let mac_version = match std::process::Command::new("sw_vers")
            .arg("-productVersion")
            .output()
        {
            Ok(output) if output.status.success() => {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            }
            _ => String::from("unknown"),
        };

        let platform_enum = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_enum().cloned())?;

        let mac_info = some_or_err!(
            platform_enum.load_named_value("macOSPlatform"),
            VmErrorReason::UnexpectedVmState.into()
        );
        let mac_info = some_or_err!(
            mac_info.as_struct(),
            VmErrorReason::UnexpectedVmState.into()
        );
        let mac_info = Object::new(mac_info);
        mac_info.write("os_build", RuntimeValue::String(mac_version.into()));

        let mac_case = some_or_err!(
            platform_enum.get_idx_of_case("macOS"),
            VmErrorReason::UnexpectedVmState.into()
        );

        let mac_enum_instance = some_or_err!(
            platform_enum.make_value(mac_case, Some(RuntimeValue::Object(mac_info))),
            VmErrorReason::UnexpectedVmState.into()
        );

        frame.stack.push(RuntimeValue::EnumValue(mac_enum_instance));
        Ok(RunloopExit::Ok(()))
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    fn eval(
        &self,
        frame: &mut Frame,
        _: &mut haxby_vm::vm::VirtualMachine,
    ) -> haxby_vm::vm::ExecutionResult<RunloopExit> {
        use haxby_vm::{
            error::vm_error::VmErrorReason, runtime_value::object::Object, some_or_err,
        };

        let platform_enum = VmBuiltins::extract_arg(frame, |x: RuntimeValue| x.as_enum().clone())?;

        let unknown_case = some_or_err!(
            platform_enum.get_idx_of_case("Unknown"),
            VmErrorReason::UnexpectedVmState.into()
        );

        let unknown_enum_instance = some_or_err!(
            platform_enum.make_value(unknown_case, None),
            VmErrorReason::UnexpectedVmState.into()
        );

        frame
            .stack
            .push(RuntimeValue::EnumValue(unknown_enum_instance));
        Ok(RunloopExit::Ok(()))
    }

    fn arity(&self) -> haxby_vm::arity::Arity {
        haxby_vm::arity::Arity::required(1)
    }

    fn attrib_byte(&self) -> u8 {
        FUNC_IS_METHOD | METHOD_ATTRIBUTE_TYPE
    }

    fn name(&self) -> &str {
        "local"
    }
}

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn dylib_haxby_inject(module: *const RuntimeModule) -> LoadResult {
    match unsafe { module.as_ref() } {
        Some(module) => {
            let platform = match module.load_named_value("Platform") {
                Some(platform) => platform,
                None => {
                    return LoadResult::error("cannot find Platform");
                }
            };

            let platform_enum = match platform.as_enum() {
                Some(platform) => platform,
                None => {
                    return LoadResult::error("Platform is not an enum");
                }
            };

            platform_enum.insert_builtin::<GetPlatformInfo>();

            LoadResult::success()
        }
        None => LoadResult::error("invalid platform module"),
    }
}
