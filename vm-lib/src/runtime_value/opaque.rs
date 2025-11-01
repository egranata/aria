// SPDX-License-Identifier: Apache-2.0
use std::{any::Any, rc::Rc};

#[derive(Clone)]
struct OpaqueValueImpl {
    val: Rc<dyn Any>,
}

#[derive(Clone)]
pub struct OpaqueValue {
    imp: OpaqueValueImpl,
}

impl std::fmt::Debug for OpaqueValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<opaque>")
    }
}

impl std::fmt::Display for OpaqueValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<opaque>")
    }
}

impl OpaqueValue {
    pub(crate) fn as_concrete_object<T: 'static>(&self) -> Option<Rc<T>> {
        self.imp.val.clone().downcast::<T>().ok()
    }

    pub fn new<T: 'static>(x: T) -> Self {
        Self {
            imp: OpaqueValueImpl { val: Rc::new(x) },
        }
    }
}
