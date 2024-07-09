fn fib(n: int) -> int {
    if (n == 1) || (n == 2) {
        return 1;
    }
    
    return fib(n - 1) + fib(n - 2)
}

fn main() {
    let fib_till: int = 10;
    let i: int = 1;
    loop {
        if i >= fib_till {
            break;
        }

        print(" " + fib(i));
        i = i + 1;
    }
    println("");
}