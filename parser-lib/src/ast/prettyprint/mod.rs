// SPDX-License-Identifier: Apache-2.0

pub mod printout_accumulator;

use crate::ast::prettyprint::printout_accumulator::PrintoutAccumulator;

pub trait PrettyPrintable {
    fn prettyprint(
        &self,
        buffer: printout_accumulator::PrintoutAccumulator,
    ) -> printout_accumulator::PrintoutAccumulator;
}

impl<T> PrettyPrintable for Box<T>
where
    T: PrettyPrintable,
{
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << self.as_ref()
    }
}
