use lang::parser::Parser;

fn main() {
    let source = "3 - 1 * 4 + 5";
    // Expected
    //(((1 + (2 * 3)) - (0 * 4)) + 5)

    // Found
    //(1 + ((2 * 3) - ((0 * 4) + 5)))

    //let source = "1 + 2 * 3 * 4 * 5 + 6 + 0 - 7 * 8";

    let mut parser = Parser::new(source);

    match parser.parse() {
        Ok(tree) => {
            println!("{}", tree);
            println!("Result: {}", tree.evalutae());
        }
        Err(e) => e.show_error(source),
    }
}
