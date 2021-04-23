extern crate cpython;

use cpython::{PyResult, Python, py_module_initializer, py_fn};

py_module_initializer!(fibrust, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "get_result", py_fn!(py, get_result(n: i32)))?;
    Ok(())
});

fn fib (n: i32) -> u64 {
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

fn get_result(_py: Python, n: i32) -> PyResult<u64> {
    Ok(fib(n))
}

