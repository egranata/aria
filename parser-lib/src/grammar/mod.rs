// SPDX-License-Identifier: Apache-2.0
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/grammar.pest"]
pub struct HaxbyParser;
