// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        SourceBuffer, StringLiteral,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

fn process_string_literal(s: &str) -> String {
    fn process_string_escapes(s: &str) -> String {
        let mut result = String::new();
        let mut chars = s.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '\\' {
                if let Some(next) = chars.peek() {
                    match next {
                        'n' => {
                            result.push('\n');
                            chars.next();
                        }
                        't' => {
                            result.push('\t');
                            chars.next();
                        }
                        '\\' => {
                            result.push('\\');
                            chars.next();
                        }
                        _ => {
                            result.push(c);
                        }
                    }
                } else {
                    result.push(c);
                }
            } else {
                result.push(c);
            }
        }

        result
    }

    let s = &s[1..s.len() - 1];
    process_string_escapes(s)
}

impl Derive for StringLiteral {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::str_literal);
        let loc = From::from(&p.as_span());
        Self {
            loc: source.pointer(loc),
            value: process_string_literal(p.as_str()),
        }
    }
}

impl PrettyPrintable for StringLiteral {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write(&self.value)
    }
}
