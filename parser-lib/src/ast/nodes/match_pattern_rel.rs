// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        Expression, MatchPatternRel, RelSymbol,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_components,
};

impl Derive for MatchPatternRel {
    gen_from_components!(match_pattern_rel; op: RelSymbol, expr: Expression);
}

impl PrettyPrintable for MatchPatternRel {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << &self.op << &self.expr
    }
}
