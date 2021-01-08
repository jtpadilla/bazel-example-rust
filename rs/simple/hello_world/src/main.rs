extern crate rs_simple_hello_lib;

fn main() {
    let hello = rs_simple_hello_lib::Greeter::new("Hello");
    hello.greet("world");
}
