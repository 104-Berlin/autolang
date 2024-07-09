use std::{env, fs::OpenOptions, io::BufReader};

use lang::{input_stream::FileInputStream, tokenizer::Tokenizer};
use source_span::{
    fmt::{Formatter, Style},
    Position, Span, DEFAULT_METRICS,
};
use utf8_chars::BufReadCharsExt;

fn main() {
    let mut args = env::args();
    args.next(); // Skip exec path
    let Some(input_file) = args.next() else {
        eprintln!("You musst provide a file to run");
        return;
    };

    let tokenizer = Tokenizer::new(FileInputStream::new(
        OpenOptions::new().read(true).open(&input_file).unwrap(),
    ));

    let mut formatter = Formatter::new();
    for token in tokenizer {
        formatter.add(token.span, Some(format!("{:?}", token.value)), Style::Note);
    }

    let mut reader = BufReader::new(OpenOptions::new().read(true).open(&input_file).unwrap());

    let full_span = Span::new(
        Position::default(),
        Position::new(usize::MAX - 1, usize::MAX - 1),
        Position::end(),
    );

    let fmt = formatter
        .render(reader.chars(), full_span, &DEFAULT_METRICS)
        .unwrap();
    println!("{}", fmt);
}
