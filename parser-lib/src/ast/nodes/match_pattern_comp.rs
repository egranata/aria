// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        CompSymbol, Expression, MatchPatternComp,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_components,
};

impl Derive for MatchPatternComp {
    gen_from_components!(match_pattern_comp; op: CompSymbol, expr: Expression);
}

impl PrettyPrintable for MatchPatternComp {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << &self.op << &self.expr
    }
}
