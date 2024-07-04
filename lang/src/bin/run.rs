use lang::{
    error::{Error, ErrorKind},
    input_stream::FileInputStream,
    parser::Parser,
};
use source_span::Span;
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

    match Parser::new(FileInputStream::new(file)).parse_expression() {
        Ok(module) => {
            println!("{}", module);
            println!("{:?}", module.evalutae());
        }
        Err(e) => {
            let file = OpenOptions::new().read(true).open(&input_file).unwrap();
            let mut reader = BufReader::new(file);

            //let mut content = std::fs::read_to_string(input_file).unwrap();

            e.show_error(
                reader
                    .chars()
                    .map(|c| c.map_err(|_| Error::new(Span::default(), ErrorKind::UnexpectedEOF))),
            );
        }
    };
}
