struct Some {
    c: int;
}

struct Test {
    a: float;
    b: Some;
}

struct IfCheck {
    val: bool;
}

fn main() {
    let t = Test {
        a: 0.1,
        b: Some {
            c: 23
        },
    };

    let x = t.a;

    if IfCheck { val: true }.val {
        println("True");
    }


    println("" + x);
}