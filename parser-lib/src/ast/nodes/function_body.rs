use crate::{
    ast::{
        CodeBlock, Expression, FunctionBody, ReturnStatement, SourceBuffer, Statement,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for FunctionBody {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::function_body);
        let loc = From::from(&p.as_span());
        let mut inner_body = p.into_inner();
        let i = inner_body.next().expect("need inner body part");
        match i.as_rule() {
            Rule::code_block => Self {
                code: CodeBlock::from_parse_tree(i, source),
            },
            Rule::expression => {
                let expr = Expression::from_parse_tree(i, source);
                let return_stmt = ReturnStatement {
                    loc: expr.loc().clone(),
                    val: Some(expr),
                };
                Self {
                    code: CodeBlock {
                        loc: source.pointer(loc),
                        entries: vec![Statement::ReturnStatement(return_stmt)],
                    },
                }
            }
            _ => panic!("Unexpected rule for inner function body: {i:?}"),
        }
    }
}

impl PrettyPrintable for FunctionBody {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer << &self.code
    }
}
