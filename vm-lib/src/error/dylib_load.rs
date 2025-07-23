// SPDX-License-Identifier: Apache-2.0
use std::ffi::{CString, c_char};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LoadStatus {
    Success = 0,
    Error = 1,
}

#[repr(C)]
pub struct LoadResult {
    pub status: LoadStatus,
    pub message: *const c_char,
}

impl LoadResult {
    pub fn success() -> Self {
        Self {
            status: LoadStatus::Success,
            message: std::ptr::null(),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            status: LoadStatus::Error,
            message: CString::new(message).unwrap().into_raw(),
        }
    }

    #[allow(unused)]
    pub(crate) fn free(self) {
        if !self.message.is_null() {
            unsafe {
                let _ = CString::from_raw(self.message as *mut c_char);
            }
        }
    }

    pub(crate) fn into_rust_string(self) -> String {
        assert!(!self.message.is_null());
        let message = unsafe { CString::from_raw(self.message as *mut c_char) };
        message.into_string().unwrap()
    }
}
