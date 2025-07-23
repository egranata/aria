// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        CodeBlock, Expression, LambaBody,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_options,
};

impl Derive for LambaBody {
    gen_from_options!(lambda_f_body; (code_block, CodeBlock), (expression, Expression));
}

impl PrettyPrintable for LambaBody {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        match self {
            LambaBody::CodeBlock(c) => c.prettyprint(buffer),
            LambaBody::Expression(e) => e.prettyprint(buffer),
        }
    }
}
