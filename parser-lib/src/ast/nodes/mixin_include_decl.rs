// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        Expression, MixinIncludeDecl,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_components,
};

impl Derive for MixinIncludeDecl {
    gen_from_components!(mixin_include_decl; what: Expression);
}

impl PrettyPrintable for MixinIncludeDecl {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "include " << &self.what
    }
}
