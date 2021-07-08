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