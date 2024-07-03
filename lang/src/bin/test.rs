use lang::parser::Parser;

fn main() {
    let source = "3 - 1 * 4 + 5";

    let mut parser = Parser::new(source);

    match parser.parse() {
        Ok(tree) => {
            println!("{}", tree);
            println!("Result: {}", tree.evalutae());
        }
        Err(e) => e.show_error(source),
    }
}
