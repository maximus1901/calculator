use std::io::stdin;
pub mod arithmetic;
pub use arithmetic::*;
pub fn secure_user_input() -> f64{
    loop {
        let mut input = String::new();
        println!("Number:");
        stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<f64>() {
            Ok(num) => {
                return num
            },
            Err(_) => {
                println!("Invalid input. Please enter a floating point number.");
                input.clear();
                continue
            }
        }
    }
}

pub fn selector_for_output(selector: i32, number1: f64, number2: f64) -> f64 {
    let mut result: f64 = 0.0;
    match selector  {
        1 => result = addition(number1, number2),
        2 => result = substraction(number1, number2),
        3 => result = multiply(number1, number2),
        4 => result = division(number1, number2),
        _ => {}
    }
    result
}