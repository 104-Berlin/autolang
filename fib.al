fn fib(n: int) -> int {
    if (n == 1) || (n == 2) {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    print("" + fib(1));
    print(", " + fib(2));
    print(", " + fib(3));
    print(", " + fib(4));
    print(", " + fib(5));
    print(", " + fib(6));
    print(", " + fib(7));
    print(", " + fib(8));
    print(", " + fib(9));
    print(", " + fib(10));
    println("");
}