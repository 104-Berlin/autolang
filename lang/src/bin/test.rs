use lang::parser::Parser;

fn main() {
    let source = "3 - 1 * (4 + 5 + 4) * 2";

    match Parser::new(source).parse_expression() {
        Ok(tree) => {
            println!("{}", tree.value);
            println!("Result: {}", tree.value.evaluate());
        }
        Err(e) => e.show_error(source.chars().map(Ok::<char, ()>)),
    }
}
