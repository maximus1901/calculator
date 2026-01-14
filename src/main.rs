use std::io::stdin;

fn main() {
    println!("Hello, world!");
    let mut flag = true;
    while flag {
        let mut selector = String::new();
        let mut number1 = String::new();
        let mut number2 = String::new();
        let mut result: f64 = 0.0;
        loop {
            println!("First Number: ");
            stdin().read_line(&mut number1).expect("Failed");
            match parse_value(&number1) {
                Some(num) => ,
                None => continue,
            }

        }
        loop{
            let mut number2 = String::new();
            println!("First Number: ");
            stdin().read_line(&mut number2).expect("Failed");
            match parse_value(&number2) {
                Some(f64) => break,
                None => continue,
            }

        }
        println!("Tell me what you want to calculate: ");
        println!("For Addition Press 1");
        println!("For Subtraction Press 2");
        println!("For Multiplication Press 3");
        println!("For Division Press 4");
        println!("For Exit Press 5");
        stdin().read_line(&mut selector).unwrap();
        let selector: i32 = selector.trim().parse().unwrap();
        match selector  {
            1 => result = addition(number1, number2),
            2 => result = substraction(number1, number2),
            3 => result = multiply(number1, number2),
            4 => result = division(number1, number2),
            5 => flag = false,
            _ => {}
        }
        println!("The Answer is: {}", result);
    }
}

fn parse_value (value: &String)-> Option<f64> {
    if let Ok(num) = value.trim().parse::<f64>() {
        Some(num)
    }
    else {
        None
    }
}
fn addition(a: f64, b: f64) -> f64 {
    a + b
}
fn substraction(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn division(a: f64, b: f64) -> f64 {
    if a == f64::from(0)|| b == f64::from(0) {
        return f64::from(0);
    }
    f64::from(a) / f64::from(b)
}
