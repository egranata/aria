// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        Primary,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_options,
};

use crate::ast::FloatLiteral;
use crate::ast::Identifier;
use crate::ast::IntLiteral;
use crate::ast::ListLiteral;
use crate::ast::ParenExpression;
use crate::ast::StringLiteral;

impl Derive for Primary {
    gen_from_options!(
        primary;
        (int_literal, IntLiteral),
        (fp_literal, FloatLiteral),
        (identifier, Identifier),
        (list_literal, ListLiteral),
        (str_literal, StringLiteral),
        (paren_expr, ParenExpression)
    );
}

impl PrettyPrintable for Primary {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        match self {
            Self::IntLiteral(il) => il.prettyprint(buffer),
            Self::FloatLiteral(fp) => fp.prettyprint(buffer),
            Self::Identifier(id) => id.prettyprint(buffer),
            Self::ListLiteral(ll) => ll.prettyprint(buffer),
            Self::StringLiteral(sl) => sl.prettyprint(buffer),
            Self::ParenExpression(pe) => pe.prettyprint(buffer),
        }
    }
}
