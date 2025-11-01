// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Arity {
    pub required: u8,
    pub optional: u8,
}

impl Arity {
    pub fn zero() -> Self {
        Self {
            required: 0,
            optional: 0,
        }
    }

    pub fn required(r: u8) -> Self {
        Self {
            required: r,
            optional: 0,
        }
    }
}
