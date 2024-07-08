fn my_func(b: float) -> float {
    let a: float = 32.32;
    a - b
}

fn main() -> float {
    let a: float = my_func(32.0);
    if a > 32.0 {
        println("Something");
    } else {
        println("False");
    }
    let b: String = "Hello World";
    123.3 + 3.1 - a
}