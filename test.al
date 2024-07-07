fn my_func(b: float) -> float {
    let a: float = 32.32;
    a - b
}

fn main() -> float {
    let a: float = my_func(32.0);
    if false {
        println("Something");
    }
    let b: String = "Hello World";
    let c: int = test(32) - 6;
    println(b + " " + a + " c: " + c);
    123.3 + 3.1 - a
}