use std::{env, fs::OpenOptions, io::BufReader};

fn main() {
    let mut args = env::args();
    args.next(); // Skip exec path
    let Some(input_file) = args.next() else {
        eprintln!("You musst provide a file to run");
        return;
    };

    let file = OpenOptions::new().read(true).open(input_file).unwrap();
    let reader = BufReader::new(file);
}
