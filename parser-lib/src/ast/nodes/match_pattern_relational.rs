// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        CompSymbol, Expression, MatchPatternRelational,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_components,
};

impl Derive for MatchPatternRelational {
    gen_from_components!(match_pattern_rel; op: CompSymbol, expr: Expression);
}

impl PrettyPrintable for MatchPatternRelational {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << &self.op << &self.expr
    }
}
