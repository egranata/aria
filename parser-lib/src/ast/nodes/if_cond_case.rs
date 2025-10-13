// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        IfCondCase,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_components,
};

use crate::ast::{Expression, MatchPatternEnumCase};

impl Derive for IfCondCase {
    gen_from_components!(if_cond_case; pattern: MatchPatternEnumCase, target: Expression);
}

impl PrettyPrintable for IfCondCase {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << &self.pattern << " = " << &self.target
    }
}
