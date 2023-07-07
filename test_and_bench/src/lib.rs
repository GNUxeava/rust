// import fib.rs by declaring module
pub mod fib;
// import the functions using "use"
pub use fib::*;
// pub use is so other crates can use it

// Testing module
#[cfg(test)] // configures for testing
mod tests {
    // import functions use super
    use super::*;
    /* to make a test function
     * #[test]
     * fn foo(bar) {
     * // test
     * }
     */
    // 1. Test if fib_one equals the right value
    #[test]
    fn test_fib_one() {
        assert_eq!(fib_one(5), 8)
    }

    // 2. Test if fib_two equals the right value
    #[test]
    fn test_fib_two() {
        assert_eq!(fib_two(7), 21)
    }

    // 3. Test if fib_one == fib_two
    #[test]
    fn test_one_eq_two() {
        let n = 10;
        // to fail
        assert_eq!(fib_one(n), fib_two(n))
    }
}

