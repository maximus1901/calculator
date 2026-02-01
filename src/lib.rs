use std::io::stdin;

pub mod arithmetic;
pub use arithmetic::*;
pub fn new_user_input() -> f64 {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input: Vec<String> = input.split_whitespace().map(|x| x.to_string()).collect();
    let mut num_stack: Vec<f64> = Vec::new();
    let mut operator_stack: Vec<String> = Vec::new();
    let mut output: f64 = 0.0;

    for token in input {
        match token.parse::<f64>() {
            Ok(num) => num_stack.push(num),
            Err(_) => {
                operator_stack.push(token.to_string())
            }
        }
    }
    for operator in operator_stack {
        let number2 = num_stack.pop().expect("num_stack stack is empty");
        let number1 = num_stack.pop().expect("num_stack stack is empty");
        let result= selector_for_output(operator.as_str(), number1, number2);
        num_stack.push(result);
    }
    output = num_stack.pop().expect("num_stack should not be empty");
    output
}

pub fn selector_for_output(selector: &str, number1: f64, number2: f64) -> f64 {
    let mut result: f64 = 0.0;
    match selector {
        "+" => result = addition(number1, number2),
        "-" => result = substraction(number1, number2),
        "*" => result = multiply(number1, number2),
        "/" => result = division(number1, number2),
        _ => {}
    }
    result
}
