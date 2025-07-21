// SPDX-License-Identifier: Apache-2.0
use crate::grammar::Rule;

use super::SourceBuffer;

pub(super) trait Derive {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self
    where
        Self: Sized;
}

#[macro_export]
macro_rules! gen_from_components {
    ( $rule:ident; $( $field:ident : $ty:ty ),* $(,)? ) => {
        fn from_parse_tree(p: pest::iterators::Pair<'_, $crate::grammar::Rule>, source: &$crate::ast::SourceBuffer) -> Self {
            assert!(p.as_rule() == $crate::grammar::Rule::$rule);
            let loc = From::from(&p.as_span());
            let mut inner = p.into_inner();
            $(
                let $field = <$ty>::from_parse_tree(
                    inner.next().expect(concat!("need ", stringify!($field))),
                    source
                );
            )*
            Self {
                loc: source.pointer(loc),
                $( $field, )*
            }
        }
    }
}

#[macro_export]
macro_rules! gen_from_options {
    ($rule:ident; $(($variant_rule:ident, $variant_type:ident)),* $(,)?) => {
        fn from_parse_tree(p: pest::iterators::Pair<'_, $crate::grammar::Rule>, source: &$crate::ast::SourceBuffer) -> Self {
            assert!(p.as_rule() == $crate::grammar::Rule::$rule);
            let mut inner = p.into_inner();
            let next = inner.next().expect("need content");
            match next.as_rule() {
                $($crate::grammar::Rule::$variant_rule => {
                    Self::$variant_type($variant_type::from_parse_tree(next, source))
                }),*
                _ => panic!("invalid node"),
            }
        }
    }
}
