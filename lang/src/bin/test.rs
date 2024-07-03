use lang::parser::Parser;

fn main() {
    let mut parser = Parser::new("32 + 123 * 43 - hello * 32 + 3");

    let tree = parser.parse();
    println!("{}", tree)
}
