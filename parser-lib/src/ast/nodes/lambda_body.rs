// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        CodeBlock, Expression, LambdaBody,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    gen_from_options,
};

impl Derive for LambdaBody {
    gen_from_options!(lambda_f_body; (code_block, CodeBlock), (expression, Expression));
}

impl PrettyPrintable for LambdaBody {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        match self {
            LambdaBody::CodeBlock(c) => c.prettyprint(buffer),
            LambdaBody::Expression(e) => e.prettyprint(buffer),
        }
    }
}
