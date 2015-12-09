#[macro_use]
extern crate async_await;

use async_await::*;

fn fib(n: i32) -> i32 {
    if n <= 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    let computation1 = async!{fib(43)};
    let computation2 = async!{fib(45)};
    let res1 = await!(computation1);
    let res2 = await!(computation2);

    println!("{}", res1);
    println!("{}", res2);
    println!("{}",  res1+res2);
}
