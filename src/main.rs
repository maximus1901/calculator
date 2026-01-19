use std::io::stdin;

fn main() {
    println!("Hello, world!");
    let mut flag = true;
    while flag {
        let mut selector = String::new();
        let mut result: f64 = 0.0;
        let number1 = secure_user_input();
        let number2 = secure_user_input();
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

fn secure_user_input() -> f64{
    loop {
        let mut input = String::new();
        println!("Number 1:");
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
