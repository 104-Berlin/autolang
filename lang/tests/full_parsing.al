fn main() {
    let a: int = 32;
    let b: float = 32.2;
    let c: bool = false;
    let d: bool = true;
    let e: String = "Some String";
    test_weird_if();
}

struct IfCheck {
    val: bool;
}

fn test_weird_if() {
    if IfCheck { val: true }.val {
        println("Check");
    }
}