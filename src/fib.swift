func fib(n: Int) -> Int {
    if n <= 2 {
        return 1
    } else {
        return (fib(n - 1) + fib(n - 2))
    }
}

let num = 45
print(fib(num))
