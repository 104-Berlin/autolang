struct Some {
    c: int;
}

struct Test {
    a: float;
    b: Some;
}


fn main() {
    let t = Test {
        a: 0.1,
        b: Some {
            c: 23
        },
    };

    let x = t.a;


    println("" + x);
}