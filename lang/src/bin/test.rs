use lang::parser::Parser;

fn main() {
    let module = Parser::new("43 * 4 + 32").parse_expression().unwrap();
    println!("{}", module);
}
