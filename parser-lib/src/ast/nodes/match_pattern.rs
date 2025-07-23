// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        MatchPattern, MatchPatternEnumCase, MatchPatternRelational,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_options,
};

impl Derive for MatchPattern {
    gen_from_options!(
        match_pattern;
        (match_pattern_rel, MatchPatternRelational),
        (match_pattern_enum_case, MatchPatternEnumCase),
    );
}

impl PrettyPrintable for MatchPattern {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        match self {
            Self::MatchPatternRelational(e) => e.prettyprint(buffer),
            Self::MatchPatternEnumCase(e) => e.prettyprint(buffer),
        }
    }
}
