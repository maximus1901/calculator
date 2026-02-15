use std::io::stdin;

pub mod arithmetic;
use crate::arithmetic::Operator::{
    Addition, Division, Modulus, Multiplication, Power, Subtraction,
};
pub use arithmetic::*;

pub fn user_input_arithmetic() -> f64 {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input: Vec<String> = input.split_whitespace().map(|x| x.to_string()).collect();
    splitting_input(input)
}

fn splitting_input(input: Vec<String>) -> f64 {
    let mut num_stack: Vec<f64> = Vec::new();
    let mut operator_stack: Vec<String> = Vec::new();

    for token in input {
        match token.parse::<f64>() {
            Ok(num) => num_stack.push(num),
            Err(_) => match token.as_str() {
                "+" => operator_stack.push(token),
                "-" => operator_stack.push(token),
                "*" => operator_stack.push(token),
                "/" => operator_stack.push(token),
                "%" => operator_stack.push(token),
                "^" => operator_stack.push(token),
                _ => panic!("Unrecognized operator number: {}", token),
            },
        }
    }
    evaluate_with_precedence(num_stack, operator_stack)
}

fn precedence(operator: String) -> i32 {
    match operator.as_str() {
        "+" | "-" => 1,
        "*" | "/" | "%" => 2,
        "^" => 3,
        _ => 0,
    }
}

fn evaluate_with_precedence(stack: Vec<f64>, operator_stack: Vec<String>) -> f64 {
    let mut values: Vec<f64> = vec![stack[0]];
    let mut ops: Vec<String> = Vec::new();

    for (i, op) in operator_stack.iter().enumerate() {
        let next_num = stack[i + 1];

        while !ops.is_empty() && precedence(ops.last().unwrap().clone()) >= precedence(op.clone()) {
            let operator = ops.pop().unwrap();
            let val2 = values.pop().unwrap();
            let val1 = values.pop().unwrap();
            values.push(arithmetic(operator, val1, val2));
        }

        ops.push(op.clone());
        values.push(next_num);
    }

    while !ops.is_empty() {
        while let Some(operator) = ops.pop() {
            let val2 = values.pop().unwrap();
            let val1 = values.pop().unwrap();
            values.push(arithmetic(operator, val1, val2));
        }
    }

    values[0]
}

fn arithmetic(selector: String, number1: f64, number2: f64) -> f64 {
    let operator = match selector.as_str() {
        "+" => Addition,
        "-" => Subtraction,
        "*" => Multiplication,
        "/" => Division,
        "%" => Modulus,
        "^" => Power,
        _ => panic!("Unknown operator: {}", selector),
    };
    operator.calculate(number1, number2).unwrap_or(0.0)
}
