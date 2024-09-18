use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
};

use lang::{compiler::Compiler, input_stream::InputStream, parser::Parser};
use miette::{Context, Error, IntoDiagnostic, NamedSource};
use virtual_machine::{
    instruction::{args::InstructionArg, Instruction},
    machine::Machine,
};

fn main() {
    let mut args = env::args();
    args.next(); // Skip exec path
    let mut step_mode = false;
    let mut input_file = None;

    for input in args {
        println!("Arg {}", input);
        if input.starts_with('-') {
            match input.as_str() {
                "-s" | "--step" => step_mode = true,
                _ => {
                    eprintln!("Unknown flag: {}", input);
                    return;
                }
            }
        } else {
            input_file = Some(input);
        }
    }

    let Some(input_file) = input_file else {
        eprintln!("You must provide a file to run");
        return;
    };

    let input = fs::read_to_string(&input_file).expect("Reading source file");

    let execution = compile(input.as_str(), step_mode);

    match execution {
        Ok(res) => {
            println!("{}", res.registers());
            println!("Stack");
            res.dump_stack();
        }
        Err(e) => {
            println!(
                "{:?}",
                e.with_source_code(NamedSource::new(input_file, input))
            );
        }
    }
}

fn compile<'a>(
    input: impl InputStream<Output = char> + 'a,
    step_mode: bool,
) -> Result<Machine, Error> {
    let module = Parser::new(input).parse_module()?;
    let (program, entry) = Compiler::default().compile(&module)?;

    println!("Entry point: {:?}", entry);

    OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("out.bin")
        .unwrap()
        .write_all(
            program
                .iter()
                .map(|i| format!("{}", Instruction::match_from_bytes(*i).unwrap()))
                .collect::<Vec<String>>()
                .join("\n")
                .as_bytes(),
        )
        .unwrap();
    Machine::default()
        .load_program(&program, entry)
        .into_diagnostic()
        .wrap_err("Loading Program")?
        .run(step_mode)
        .into_diagnostic()
        .wrap_err("Running program")
}

/* use lang::{execution::ExecutionContext, input_stream::FileInputStream, parser::Parser};
use std::{env, fs::OpenOptions, io::BufReader};
use utf8_chars::BufReadCharsExt;
use lang::{execution::ExecutionContext, parser::Parser};
use miette::NamedSource;
use std::{
    env,
    fs::{self},
};

fn main() {
    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .without_syntax_highlighting()
                .context_lines(4)
                .build(),
        )
    }))
    .expect("Failed to set miette hook");

    let mut args = env::args();
    args.next(); // Skip exec path
    let Some(input_file) = args.next() else {
        eprintln!("You must provide a file to run");
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
            eprintln!(
                "{:?}",
                e.with_source_code(NamedSource::new(input_file, input))
            );
        }
    };
}
*/
