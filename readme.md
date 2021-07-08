//! This is a simple crate to illustrate number conversion from float to integer.


//! Install this crate with `cargo install convertnumber`.
//! To convert your number, pass it from the command line interface. For instance, Run `cargo run 20.10` to get the output `21`.
/// I was trying out `impl` and `traits` and thought it was cool to build something from it.
/// I'm using the Rust `ceil` method to round up Rust numbers. Since I'm collecting `args` with Strings `collect` method, args is String.
/// I use the `parse` method to parse String to integer, then finally round it up to i32.
/// For now, this can only convert `i64` type to `i32`. The goal in future is to convert an `f32` or `f64` type to `i32` type.