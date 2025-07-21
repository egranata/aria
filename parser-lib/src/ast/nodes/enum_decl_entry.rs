// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        derive::Derive,
        prettyprint::{printout_accumulator::PrintoutAccumulator, PrettyPrintable},
        EnumDeclEntry,
    },
    gen_from_options,
};

use crate::ast::{EnumCaseDecl, StructEntry};

impl Derive for EnumDeclEntry {
    gen_from_options!(enum_decl_entry; (enum_case_decl, EnumCaseDecl), (struct_entry, StructEntry));
}

impl PrettyPrintable for EnumDeclEntry {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        match self {
            Self::EnumCaseDecl(e) => e.prettyprint(buffer),
            Self::StructEntry(s) => s.prettyprint(buffer),
        }
    }
}
