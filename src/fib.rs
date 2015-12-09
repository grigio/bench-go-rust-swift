fn fib(n: i64) -> i64 {
    if n <= 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    let num = 45;
    println!("{}", fib(num))
}
