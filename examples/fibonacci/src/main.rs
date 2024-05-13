fn fib(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn fib_non_recursive(n: u64) -> u64 {
    if n < 2 {
        return n;
    }

    let mut last_last_value = 0u64;
    let mut last_value = 1u64;
    for _ in 1..n {
        let tmp = last_last_value;
        last_last_value = last_value;
        last_value = last_last_value + tmp;
    }
    last_value
}
fn main() {
    let n = 10;
    println!("fib({n}) = {}", fib(n));
    println!("fib({n}) = {}", fib_non_recursive(n));
}
