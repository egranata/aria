// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        PostfixExpression, PostfixRvalue, SourceBuffer, UnaryOperation, UnarySymbol,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for UnaryOperation {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::unary);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        if inner.len() == 1 {
            let postfix = inner.next().expect("need postfix");
            let postfix = PostfixRvalue::from_parse_tree(postfix, source);
            Self {
                loc: source.pointer(loc),
                operand: None,
                postfix,
            }
        } else if inner.len() == 2 {
            let operand = inner.next().expect("need operand");
            let operand = UnarySymbol::from_parse_tree(operand, source);
            let postfix = inner.next().expect("need postfix");
            let postfix = PostfixRvalue::from_parse_tree(postfix, source);

            if operand == UnarySymbol::Minus
                && postfix.expr.terms.is_empty()
                && let Some(il) = postfix.expr.base.as_int_literal()
                && il.base == crate::ast::IntLiteralBase::Decimal
            {
                let new_val = if il.val.starts_with('-') {
                    il.val.strip_prefix('-').unwrap_or(&il.val).to_string()
                } else {
                    format!("-{}", il.val)
                };
                let new_il = crate::ast::IntLiteral {
                    loc: il.loc.clone(),
                    base: il.base.clone(),
                    val: new_val,
                };
                return Self {
                    loc: source.pointer(loc),
                    operand: None,
                    postfix: PostfixRvalue {
                        loc: postfix.loc.clone(),
                        expr: PostfixExpression {
                            loc: postfix.expr.loc.clone(),
                            base: crate::ast::Primary::IntLiteral(new_il),
                            terms: vec![],
                        },
                    },
                };
            }

            Self {
                loc: source.pointer(loc),
                operand: Some(operand),
                postfix,
            }
        } else {
            panic!("unexpected unary operation structure");
        }
    }
}

impl PrettyPrintable for UnaryOperation {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << &self.operand << &self.postfix
    }
}
