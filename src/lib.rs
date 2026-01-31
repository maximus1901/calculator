use std::io::stdin;
pub mod arithmetic;
pub use arithmetic::*;
pub fn new_user_input() -> f64 {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input: Vec<String> = input.split_whitespace().map(|x| x.to_string()).collect();
    let mut stack: Vec<f64> = Vec::new();
    let mut output: f64 = 0.0;

    for token in input {
        match token.parse::<f64>() {
            Ok(num) => stack.push(num),
            Err(_) => {
                let b = stack.pop().expect("Nicht genügend Zahlen auf dem Stack!");
                let a = stack.pop().expect("Nicht genügend Zahlen auf dem Stack!");
                let operator = token.to_string();
                output = selector_for_output(operator, b, a);
            }
        }
    }
    output
}
pub fn selector_for_output(selector: String, number1: f64, number2: f64) -> f64 {
    let mut result: f64 = 0.0;
    match selector.as_str() {
        "+" => result = addition(number1, number2),
        "-" => result = substraction(number1, number2),
        "*" => result = multiply(number1, number2),
        "/" => result = division(number1, number2),
        _ => {}
    }
    result
}
