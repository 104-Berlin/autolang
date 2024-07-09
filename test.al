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
    print(", " + fib(11));
    print(", " + fib(12));
    print(", " + fib(13));
    print(", " + fib(14));
    print(", " + fib(15));
    print(", " + fib(16));
    print(", " + fib(17));
    print(", " + fib(18));
    print(", " + fib(19));
    print(", " + fib(20));
    print(", " + fib(21));
    print(", " + fib(22));
    println("");
}