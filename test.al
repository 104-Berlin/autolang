struct Some {
    c: int;
}

struct Test {
    a: int;
    b: Some;
}


fn main() {
    let t: Test = Test {
        a: 32;
        b: Some {
            c: 23
        },
    };

    let x:int = t.a;


    println("" + x);
}