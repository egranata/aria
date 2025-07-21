use std::collections::HashSet;

// SPDX-License-Identifier: Apache-2.0
use crate::runtime_value::{mixin::Mixin, RuntimeValue};

#[derive(Default)]
pub struct MixinIncluder {
    mixins: Vec<Mixin>,
}

impl MixinIncluder {
    pub fn load_named_value(&self, name: &str) -> Option<RuntimeValue> {
        self.mixins
            .iter()
            .rev()
            .find_map(|mixin| mixin.load_named_value(name))
    }

    pub fn include(&mut self, mixin: Mixin) {
        self.mixins.push(mixin);
    }

    pub fn contains(&self, mixin: &Mixin) -> bool {
        for m in &self.mixins {
            if m.isa_mixin(mixin) {
                return true;
            }
        }
        false
    }

    pub fn list_attributes(&self) -> HashSet<String> {
        let mut attrs = HashSet::new();
        for mixin in &self.mixins {
            attrs.extend(mixin.list_attributes());
        }
        attrs
    }
}
