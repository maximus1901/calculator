use std::collections::VecDeque;
use std::io::stdin;

pub mod arithmetic;
use crate::arithmetic::Operator::{
    addition, division, modulus, multiplication, power, subtraction,
};
pub use arithmetic::*;

pub fn user_input_arithmetic() -> f64 {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input: Vec<String> = input.split_whitespace().map(|x| x.to_string()).collect();
    arithmetic(input)
}

pub fn arithmetic(input: Vec<String>) -> f64 {
    let mut num_stack: VecDeque<f64> = VecDeque::new();
    let mut operator_stack: VecDeque<String> = VecDeque::new();
    let mut priority_operator_stack: VecDeque<String> = VecDeque::new();

    for token in input {
        match token.parse::<f64>() {
            Ok(num) => num_stack.push_back(num),
            Err(_) => match token.as_str() {
                "+" => operator_stack.push_back(token),
                "-" => operator_stack.push_back(token),
                "*" => priority_operator_stack.push_back(token),
                "/" => priority_operator_stack.push_back(token),
                "%" => priority_operator_stack.push_back(token),
                "^" => priority_operator_stack.push_back(token),
                _ => {}
            },
        }
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
    let operator = match selector {
        "+" => addition,
        "-" => subtraction,
        "*" => multiplication,
        "/" => division,
        "%" => modulus,
        "^" => power,
        _ => panic!("Unknown operator: {}", selector),
    };
    operator.calculate(number1, number2).unwrap_or(0.0)
}
