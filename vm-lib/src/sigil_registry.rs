use crate::runtime_value::RuntimeValue;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct SigilRegistry {
    registry: Rc<RefCell<HashMap<String, RuntimeValue>>>,
}

impl SigilRegistry {
    pub fn register_sigil(&self, name: &str, function: RuntimeValue) {
        self.registry.borrow_mut().insert(name.to_owned(), function);
    }

    pub fn lookup_sigil(&self, name: &str) -> Option<RuntimeValue> {
        self.registry.borrow().get(name).cloned()
    }
}

impl Default for SigilRegistry {
    fn default() -> Self {
        Self {
            registry: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}
