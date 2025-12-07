// SPDX-License-Identifier: Apache-2.0
use crate::do_compile::{
    CompilationResult, CompileNode, CompileParams, Expression, Identifier, MixinIncludeDecl,
    Primary, StructEntry, emit_type_members_compile,
};

impl<'a> CompileNode<'a> for aria_parser::ast::ExtensionDecl {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        self.target.do_compile(params)?;

        // If the extension has `: MixinName` syntax, inject an include at the start
        if let Some(ref mixin_name) = self.inherits {
            let mixin_include = StructEntry::MixinInclude(Box::new(MixinIncludeDecl {
                loc: self.loc.clone(),
                what: Expression::from(&Primary::Identifier(Identifier {
                    loc: mixin_name.loc.clone(),
                    value: mixin_name.value.clone(),
                })),
            }));

            let mut new_body = vec![mixin_include];
            new_body.extend_from_slice(&self.body);

            emit_type_members_compile(&new_body, params, true)
        } else {
            emit_type_members_compile(&self.body, params, true)
        }
    }
}
