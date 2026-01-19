use std::io::stdin;
use calculator::{secure_user_input, selector_for_output};
fn main() {
    println!("Hello, world!");
    loop{
        let mut selector = String::new();
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
        let result = selector_for_output(selector, number1, number2);
        if selector == 5{
            break;
        }
        println!("The Answer is: {}", result);
    }
}