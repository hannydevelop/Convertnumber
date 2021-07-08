//! This is a simple crate to illustrate number conversion from float to integer.
//! I was trying out `impl` and `traits` and thought it was cool to build something from it.
//! I'm using the Rust `ceil` method to round up Rust numbers. Since I'm collecting `args` with Strings `collect` method, args is String.
//! I use the `parse` method to parse String to integer, then finally round it up to i32.
//! For now, this can only convert `i64` type to `i32`. The goal in future is to convert an `f32` or `f64` type to `i32` type.


//! Install this crate with `cargo install convertnumber`.
//! To convert your number, pass it from the command line interface. For instance, Run `cargo run 20.10` to get the output `21`.

use std::env;

trait ConvertNumber<T> {
    fn convert_number(&self) -> T;
}

struct Integer32 {
    number: i64,
}

impl ConvertNumber<i64> for Integer32 {
    fn convert_number(&self) -> i64 {
        self.number.clone()
    }
}

fn convert_number<T: ConvertNumber<i64>>(t: &T) {
    println!("Your float number is now an integer: {}", t.convert_number())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let number = &args[1];

    let parse_number: f64 = number.parse().unwrap();

    let integer_32 = Integer32 {number: parse_number.ceil() as i64};

    convert_number(&integer_32);
}