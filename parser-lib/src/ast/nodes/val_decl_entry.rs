// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        DeclarationId, Expression, ValDeclEntry,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_components,
};

impl Derive for ValDeclEntry {
    gen_from_components!(val_decl_entry; id: DeclarationId, val: Expression);
}

impl PrettyPrintable for ValDeclEntry {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << &self.id << " = " << &self.val
    }
}
