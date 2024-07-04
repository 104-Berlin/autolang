use lang::{input_stream::FileInputStream, parser::Parser};
use std::{env, fs::OpenOptions, io::BufReader};
use utf8_chars::BufReadCharsExt;

fn main() {
    let mut args = env::args();
    args.next(); // Skip exec path
    let Some(input_file) = args.next() else {
        eprintln!("You musst provide a file to run");
        return;
    };

    let file = OpenOptions::new().read(true).open(&input_file).unwrap();
    /*let mut input_stream = Tokenizer::new(FileInputStream::new(file));
        for tok in input_stream {
        println!("{:?}", tok);
    }*/

    match Parser::new(FileInputStream::new(file)).parse_module() {
        Ok(module) => {
            for func in module.value.functions() {
                let func = &func.value;
                println!("Function: {}", func.proto.value.name.value);
                for stmt in func.proto.value.arguments.value.iter() {
                    println!("arg {}: {};", stmt.0.value, stmt.1.value);
                }
                println!("Body: {}", func.body.value);
            }
        }
        Err(e) => {
            let file = OpenOptions::new().read(true).open(&input_file).unwrap();
            let mut reader = BufReader::new(file);

            e.show_error(reader.chars().map(|c| c.map_err(|_| ())));
        }
    };
}
