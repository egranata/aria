// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        CodeBlock, Identifier, TryBlock,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_components,
};

impl Derive for TryBlock {
    gen_from_components!(try_block; body: CodeBlock, id: Identifier, catch: CodeBlock);
}

impl PrettyPrintable for TryBlock {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << "try " << &self.body << " catch (" << &self.id << ") " << &self.catch
    }
}
