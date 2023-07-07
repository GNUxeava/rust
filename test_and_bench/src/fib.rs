// import threads from standard lib
use std::thread::*;
pub fn fib_one(n: u32) -> u32 {
    // spawn a thread
    let thr = spawn(move || {
        // match n
        match n {
            // if n <= 1
            0 => 1,
            1 => 1,
            // else 
            _ => fib_one(n - 1) + fib_one(n - 2)
        }
    });
    // join the thread to get the result
    thr.join().expect("Couldn't join threads.")
}

pub fn fib_two(n: u32) -> u32 {
    if n<= 1 {
        1
    } else {
        fib_two(n - 1) + fib_two(n - 2)
    }
}
