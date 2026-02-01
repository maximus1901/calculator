use std::io::stdin;

pub mod arithmetic;
pub use arithmetic::*;

pub fn user_input() -> Vec<String> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input: Vec<String> = input.split_whitespace().map(|x| x.to_string()).collect();
    input
}

pub fn arithmetic() -> f64 {
    let input = user_input();
    let mut num_stack: Vec<f64> = Vec::new();
    let mut operator_stack: Vec<String> = Vec::new();

    //dividing input onto two stack
    for token in input {
        match token.parse::<f64>() {
            Ok(num) => num_stack.push(num),
            Err(_) => {
                operator_stack.push(token.to_string())
            }
        }
    }

    //operating on this two stack
    for operator in operator_stack {
        let number2 = num_stack.pop().expect("num_stack stack is empty");
        let number1 = num_stack.pop().expect("num_stack stack is empty");
        let result= selector_for_output(operator.as_str(), number1, number2);
        num_stack.push(result);
    }
    num_stack[0]
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
