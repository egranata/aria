// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        MatchPattern, MatchPatternComp, MatchPatternEnumCase, MatchPatternRel,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_options,
};

impl Derive for MatchPattern {
    gen_from_options!(
        match_pattern;
        (match_pattern_comp, MatchPatternComp),
        (match_pattern_enum_case, MatchPatternEnumCase),
        (match_pattern_rel, MatchPatternRel),
    );
}

impl PrettyPrintable for MatchPattern {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        match self {
            Self::MatchPatternComp(e) => e.prettyprint(buffer),
            Self::MatchPatternRel(e) => e.prettyprint(buffer),
            Self::MatchPatternEnumCase(e) => e.prettyprint(buffer),
        }
    }
}
