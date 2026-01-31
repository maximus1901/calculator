use calculator::new_user_input;
fn main() {
    println!("Hello, world!");
    loop {
        println!("Tell me what you want to calculate: ");
        let result = new_user_input();
        print!("Your Input accumulates to: {:?}", result);
    }
}
