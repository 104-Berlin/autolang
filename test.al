// This is my single line comment
/*
This is a multiline comment.
Second line*/

fn my_func(b: float) -> float {
    let a: float = 32.32;
    a - b
}

fn main() -> float {
    let a: float = my_func(32.0);
    123.3 + 3.1 - a
}