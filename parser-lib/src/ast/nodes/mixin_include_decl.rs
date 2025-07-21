// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        Expression, MixinIncludeDecl,
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
