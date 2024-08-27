use std::{
    env,
    fs::{self},
};

use lang::tokenizer::Tokenizer;
use miette::{diagnostic, LabeledSpan};

fn main() {
    let mut args = env::args();
    args.next(); // Skip exec path
    let Some(input_file) = args.next() else {
        eprintln!("You musst provide a file to run");
        return;
    };

    let input = fs::read_to_string(&input_file).expect("Reading source file");

    let tokenizer = Tokenizer::new(input.as_str())
        .map(|tok| LabeledSpan::at(tok.span, format!("{}", tok.value)))
        .collect::<Vec<_>>();

    let report: miette::Report = diagnostic!(labels = tokenizer, "Tokenized input").into();
    eprintln!("{:?}", report.with_source_code(input));
}
