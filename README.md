# Python Taiwan 2021 - Talk: Python speed up with Rust
## Summary

Let's begin with the famous Fibonacci Sequence. By the definition:
```
Fib(0) = 0
Fib(1) = 1
Fib(n) = Fib(n-1) + Fib(n-2)
```
The time complexity is O(2^n) when we calculate it with recursive algorithm. It is perfect for us to observe the performance improvement when using rust to speed up python code.

## Pure Python

Folder: Python/Pure

fib.py
```
def fib(n):
    if n < 0:
        raise ValueError("n must greater or equal 0.")
    if n == 0:
        return 0
    if n == 1:
        return 1
    return fib(n-1) + fib(n-2)
```

**Try it**

```
> python3 sol1.py 10
Fib(10) = 55
```

## Pure Rust

Folder: Rust/fib

fib.rs
```
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
```

**Build**
```
> cd Rust/fib

Rust/fib> cargo build
```

**Try it**
```
> ./fib 10
Fib(10) = 55
```

## Python with Rust library

Folder: Python/Hybird

**Build**
```
> cd Python/Hybrid/fibrust

Python/Hybrid/fibrust> cargo build --release

Python/Hybrid/fibrust> cd ..

Python/Hybrid> mv fibrust/target/release/libfibrust.so fibrust.so
```

**Try it**
```
> python3 sol2.py 10
Fib(10) = 55
```
