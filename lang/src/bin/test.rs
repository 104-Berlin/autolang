use lang::{error::Error, parser::Parser};

fn main() {
    let source = "3 - 1 * (4 + 5 + 4) * 2";

    match Parser::new(source).parse_expression() {
        Ok(tree) => {
            println!("{}", tree);
            println!("Result: {}", tree.evaluate());
        }
        Err(e) => e.show_error(source.chars().map(Ok::<char, Error>)),
    }
}
