use lang::parser::{parse_expression, spans::InputSpan};

fn main() {
    let input = "1 * 2 + 3 + 4 * 5 - funccall()";
    match parse_expression(InputSpan::new(input)) {
        Ok(module) => println!("{}", module),
        Err(e) => {
            let code = &input
                [e.span().location_offset()..e.span().location_offset() + *e.span().fragment()];
            println!("Error parsing Expression: {}\n{}", e, code)
        }
    }
}
