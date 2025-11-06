// SPDX-License-Identifier: Apache-2.0
use crate::ast::{
    IntLiteral, IntLiteralBase,
    derive::Derive,
    prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
};

impl Derive for IntLiteral {
    fn from_parse_tree(
        p: pest::iterators::Pair<'_, crate::grammar::Rule>,
        source: &crate::ast::SourceBuffer,
    ) -> Self {
        assert!(p.as_rule() == crate::grammar::Rule::int_literal);
        let loc = From::from(&p.as_span());
        let val = p.as_str().to_owned();
        let base = match p
            .into_inner()
            .next()
            .expect("need a literal content")
            .as_rule()
        {
            crate::grammar::Rule::bin_int_literal => IntLiteralBase::Binary,
            crate::grammar::Rule::oct_int_literal => IntLiteralBase::Octal,
            crate::grammar::Rule::dec_int_literal => IntLiteralBase::Decimal,
            crate::grammar::Rule::hex_int_literal => IntLiteralBase::Hexadecimal,
            _ => panic!("unexpected int literal base"),
        };
        Self {
            loc: source.pointer(loc),
            base,
            val,
        }
    }
}

impl PrettyPrintable for IntLiteral {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write(&self.val)
    }
}
