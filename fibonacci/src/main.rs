fn fib(n: u32) -> u32 {
    if (n <= 1) {
       return 1;
    }
    fib(n - 1) + fib(n - 2)
}

fn main() {
    println!("{}", fib(5));
}
