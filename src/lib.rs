use std::io::stdin;
use std::collections::VecDeque;
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
    let mut num_stack: VecDeque<f64> = VecDeque::new();
    let mut operator_stack: VecDeque<String> = VecDeque::new();
    let mut priority_operator_stack: VecDeque<String> = VecDeque::new();

    //dividing input onto two stack
    for token in input {
        match token.parse::<f64>() {
            Ok(num) => num_stack.push_back(num),
            Err(_) => {
                match token.as_str() {
                    "+" => operator_stack.push_back(token),
                    "-" => operator_stack.push_back(token),
                    "*" => priority_operator_stack.push_back(token),
                    "/" => priority_operator_stack.push_back(token),
                    "%" => priority_operator_stack.push_back(token),
                    "^" => priority_operator_stack.push_back(token),
                    _ => {}
                }
            }
        }
    }
    for operator in priority_operator_stack{

    }

    //operating on this two stack
    for operator in operator_stack {
        let number1 = num_stack.pop_front().expect("num_stack stack is empty");
        let number2 = num_stack.pop_front().expect("num_stack stack is empty");
        let result = selector_for_output(operator.as_str(), number1, number2);
        num_stack.push_front(result);
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
        "%" => result = modulo(number1, number2),
        "^" => result = pow(number1, number2),
        _ => {}
    }
    result
}
