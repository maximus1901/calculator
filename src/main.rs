use calculator::{arithmetic};
fn main() {
    println!("Hello, world!");
    loop {
        println!("Tell me what you want to calculate: ");
        let result = arithmetic();
        print!("Your Input accumulates to: {:?}", result);
    }
}
