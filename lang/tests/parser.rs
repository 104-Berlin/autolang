use lang::{execution::ExecutionContext, parser::Parser};

#[test]
fn test_full_language_parser() {
    let input = include_str!("full_parsing.al");
    let execution = Parser::new(input).parse_module().and_then(|module| {
        let mut ctx = ExecutionContext::new(&module);
        ctx.execute()
    });

    match execution {
        Ok(_) => {}
        Err(e) => {
            e.show_error(input.chars().map(|c| Ok(c)));
            panic!("Error during parsing");
        }
    };
}
