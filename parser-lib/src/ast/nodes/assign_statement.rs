// SPDX-License-Identifier: Apache-2.0
use crate::ast::{
    AssignStatement, Expression, PostfixExpression,
    derive::Derive,
    prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
};

impl Derive for AssignStatement {
    fn from_parse_tree(
        p: pest::iterators::Pair<'_, crate::grammar::Rule>,
        source: &crate::ast::SourceBuffer,
    ) -> Self {
        assert!(p.as_rule() == crate::grammar::Rule::val_write_stmt);
        let loc = From::from(&p.as_span());
        let inner = p.into_inner();
        let mut id = vec![];
        let mut val = vec![];
        for next in inner {
            match next.as_rule() {
                crate::grammar::Rule::postfix_lv => {
                    id.push(<PostfixExpression>::from_parse_tree(next, source));
                }
                crate::grammar::Rule::expression => {
                    val.push(<Expression>::from_parse_tree(next, source));
                }
                _ => panic!("unexpected token, want postfix_lv or expression"),
            }
        }
        Self {
            loc: source.pointer(loc),
            id,
            val,
        }
    }
}

impl PrettyPrintable for AssignStatement {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer
            .write_separated_list(&self.id, ", ")
            .write(" = ")
            .write_separated_list(&self.val, ", ")
            << ";"
    }
}
