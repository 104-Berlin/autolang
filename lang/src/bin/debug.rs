use lang::{compiler::Compiler, input_stream::InputStream, parser::Parser};
use miette::{Context, Error, IntoDiagnostic};
use virtual_machine::machine::Machine;

fn main() {
    let input = "fn main() { let z = 0; loop { break; } }";

    if let Err(e) = compile(input, false) {
        println!("{:?}", e.with_source_code(input));
    }
}

fn compile<'a>(
    input: impl InputStream<Output = char> + 'a,
    step_mode: bool,
) -> Result<Machine, Error> {
    let module = Parser::new(input).parse_module()?;
    let program = Compiler::default().compile(&module)?;

    Machine::default()
        .load_program(&program)
        .into_diagnostic()
        .wrap_err("Loading Program")?
        .run(step_mode)
        .into_diagnostic()
        .wrap_err("Running program")
}
