pub fn fib (n: i32) -> u64 {
    if n < 0 {
        panic!("n must greater or equal 0.");
    }
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return fib(n - 1) + fib(n - 2)
}
