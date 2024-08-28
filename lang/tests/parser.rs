use lang::{execution::ExecutionContext, parser::Parser};

#[test]
fn test_full_language_parser() {
    let input = include_str!("full_parsing.al");
    Parser::new(input)
        .parse_module()
        .and_then(|module| {
            let mut ctx = ExecutionContext::new(&module);
            ctx.execute()
        })
        .unwrap();
}
