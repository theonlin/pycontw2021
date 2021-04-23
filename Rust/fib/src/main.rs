use std::env;

mod fib;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let n: i32 = args[1].parse::<i32>().unwrap();
        println!("Fib({}) = {}", n, fib::fib(n));
    }
}
