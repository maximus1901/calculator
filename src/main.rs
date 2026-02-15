use calculator::user_input_arithmetic;
fn main() {
    println!("Hello, world!");
    loop {
        println!("Tell me what you want to calculate: ");
        let result = user_input_arithmetic();
        println!("Your Input accumulates to: {:?}", result);
    }
}
