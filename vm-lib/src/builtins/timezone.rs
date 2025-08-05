// SPDX-License-Identifier: Apache-2.0
use crate::{
    builtins::VmBuiltins,
    frame::Frame,
    runtime_value::{RuntimeValue, function::BuiltinFunctionImpl, list::List},
    vm::RunloopExit,
};

#[derive(Default)]
struct TimezoneInfo {}
impl BuiltinFunctionImpl for TimezoneInfo {
    fn eval(
        &self,
        cur_frame: &mut Frame,
        _: &mut crate::vm::VirtualMachine,
    ) -> crate::vm::ExecutionResult<RunloopExit> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("before the epoch")
            .as_secs() as i64;

        let mut tm = libc::tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: std::ptr::null_mut(),
        };
        unsafe {
            libc::localtime_r(&now, &mut tm);
        }

        let tm_zone_name = if !tm.tm_zone.is_null() {
            "unknown"
        } else {
            unsafe {
                std::ffi::CStr::from_ptr(tm.tm_zone)
                    .to_str()
                    .unwrap_or("unknown")
            }
        };

        let resulting_object = List::from(&[]);
        resulting_object.append(RuntimeValue::Integer((tm.tm_gmtoff / 60).into()));
        resulting_object.append(RuntimeValue::String(tm_zone_name.to_string().into()));

        cur_frame.stack.push(RuntimeValue::List(resulting_object));
        Ok(RunloopExit::Ok(()))
    }

    fn arity(&self) -> u8 {
        0_u8
    }

    fn name(&self) -> &str {
        "timezone_info"
    }
}

pub(super) fn insert_builtins(builtins: &mut VmBuiltins) {
    builtins.insert_builtin::<TimezoneInfo>();
}
