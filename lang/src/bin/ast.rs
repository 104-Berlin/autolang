use std::{env, fs::OpenOptions, io::BufReader};

use lang::{input_stream::FileInputStream, parser::Parser};

use utf8_chars::BufReadCharsExt;

fn main() {
    let mut args = env::args();
    args.next(); // Skip exec path
    let Some(input_file) = args.next() else {
        eprintln!("You musst provide a file to run");
        return;
    };

    match Parser::new(FileInputStream::new(
        OpenOptions::new().read(true).open(&input_file).unwrap(),
    ))
    .parse_module()
    {
        Ok(module) => {
            for func in module.value.functions() {
                println!("{}", func.value);
            }
        }
        Err(e) => {
            let file = OpenOptions::new().read(true).open(&input_file).unwrap();
            let mut reader = BufReader::new(file);

            e.show_error(reader.chars().map(|c| c.map_err(|_| ())));
        }
    }
}
