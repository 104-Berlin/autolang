use lang::{execution::ExecutionContext, parser::Parser};
use std::{
    env,
    fs::{self},
};

fn main() {
    let mut args = env::args();
    args.next(); // Skip exec path
    let Some(input_file) = args.next() else {
        eprintln!("You musst provide a file to run");
        return;
    };

    let input = fs::read_to_string(&input_file).expect("Reading source file");
    /*let mut input_stream = Tokenizer::new(FileInputStream::new(file));
        for tok in input_stream {
        println!("{:?}", tok);
    }*/

    let execution = Parser::new(input.as_str())
        .parse_module()
        .and_then(|module| {
            let mut ctx = ExecutionContext::new(&module);
            ctx.execute()
        });

    match execution {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{:?}", e.with_source_code(input));
        }
    };
}
