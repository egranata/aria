// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        IfCondExpr,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_options,
};

use crate::ast::{Expression, IfCondCase};

impl Derive for IfCondExpr {
    gen_from_options!(if_cond; (if_cond_case, IfCondCase), (expression, Expression));
}

impl PrettyPrintable for IfCondExpr {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        match self {
            Self::IfCondCase(c) => c.prettyprint(buffer),
            Self::Expression(e) => e.prettyprint(buffer),
        }
    }
}
