pub mod test {
    // adds hello to string
    fn add_hello(str: &str) -> String {
        let mut owned = str.to_owned();
        owned.push_str("hello");
        owned
    }

    // adds world to string
    fn add_world(str: &str) -> String {
        let mut owned = str.to_owned();
        owned.push_str(" world");
        owned
    }

    pub fn hello_world(s: &str) {
        // adds hello
        let with_hello = self::add_hello(s);
        // adds world
        let hw = self::add_world(&with_hello);
        println!("{}", hw);
    }
}

fn main() {
    let s = "I am Bob.";
    // access test using test::<method>
    test::hello_world(s);
}
