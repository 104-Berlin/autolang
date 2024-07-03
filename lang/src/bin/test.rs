use lang::{parser::Parser, tokenizer::Tokenizer};

fn main() {
    let source = "32 + 123 * 43 -- hello * 32 + 3";

    let tokens = Tokenizer::new(source).collect::<Vec<_>>();
    println!("{:#?}", tokens);

    let mut parser = Parser::new(source);

    match parser.parse() {
        Ok(tree) => println!("{}", tree),
        Err(e) => e.show_error(source),
    }
    
}
