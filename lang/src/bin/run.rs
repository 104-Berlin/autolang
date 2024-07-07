use lang::{execution::ExecutionContext, input_stream::FileInputStream, parser::Parser};
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

    let execution = Parser::new(FileInputStream::new(file))
        .parse_module()
        .and_then(|module| {
            let mut ctx = ExecutionContext::new(&module);
            ctx.execute()
        });

    match execution {
        Ok(_) => {}
        Err(e) => {
            let file = OpenOptions::new().read(true).open(&input_file).unwrap();
            let mut reader = BufReader::new(file);

            e.show_error(reader.chars().map(|c| c.map_err(|_| ())));
        }
    };
}
