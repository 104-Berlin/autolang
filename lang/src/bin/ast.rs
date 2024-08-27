use std::{
    env,
    fs::{self},
};

use lang::parser::Parser;

fn main() {
    let mut args = env::args();
    args.next(); // Skip exec path
    let Some(input_file) = args.next() else {
        eprintln!("You musst provide a file to run");
        return;
    };

    let input = fs::read_to_string(input_file).unwrap();

    let parsed = Parser::new(input.as_str()).parse_module();

    match parsed {
        Ok(module) => {
            for func in module.value.functions() {
                println!("{}", func.value);
            }
        }
        Err(e) => {
            println!("{:?}", e.with_source_code(input.clone()));
        }
    }
}
