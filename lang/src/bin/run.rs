use std::{
    env,
    fs::OpenOptions,
    io::{BufReader, Write},
};

use lang::{
    compiler::Compiler,
    error::ALError,
    input_stream::{FileInputStream, InputStream},
    parser::Parser,
};
use utf8_chars::BufReadCharsExt;
use virtual_machine::{
    instruction::{args::InstructionArg, Instruction},
    machine::Machine,
};

fn main() {
    let mut args = env::args();
    args.next(); // Skip exec path
    let Some(input_file) = args.next() else {
        eprintln!("You must provide a file to run");
        return;
    };

    let file = OpenOptions::new().read(true).open(&input_file).unwrap();

    let execution = compile(FileInputStream::new(file));

    match execution {
        Ok(res) => {
            println!("{}", res.registers());
            println!("Stack");
            res.dump_stack();
        }
        Err(e) => {
            let file = OpenOptions::new().read(true).open(&input_file).unwrap();
            let mut reader = BufReader::new(file);

            e.show_error(reader.chars().map(|c| c.map_err(|_| ())));
        }
    }
}

fn compile(input: impl InputStream<Output = char> + 'static) -> Result<Machine, ALError> {
    let module = Parser::new(input).parse_module()?;
    let program = Compiler::default().compile(&module)?;
    OpenOptions::new()
        .write(true)
        .create(true)
        .open("out.bin")
        .unwrap()
        .write(
            &program
                .iter()
                .map(|i| format!("{}", Instruction::match_from_bytes(*i).unwrap()))
                .collect::<Vec<String>>()
                .join("\n")
                .as_bytes(),
        )
        .unwrap();
    Ok(Machine::new().load_program(&program)?.run(false)?)
}

/* use lang::{execution::ExecutionContext, input_stream::FileInputStream, parser::Parser};
use std::{env, fs::OpenOptions, io::BufReader};
use utf8_chars::BufReadCharsExt;

fn main() {
    let mut args = env::args();
    args.next(); // Skip exec path
    let Some(input_file) = args.next() else {
        eprintln!("You must provide a file to run");
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
*/
