mod model {
    pub mod test_demo;
}

use rand::Rng;
use crate::model::test_demo::test_demo::test_demo;

fn main() {
    let input = get_user_input();
    print_user_input(input);
    let x = test_demo();
    println!("{}", x);
}

fn get_user_input() -> String {
    println!("Please input your text:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    return input;
}

fn print_user_input(input: String) {
    println!("your input value is: {}", input);
}

fn create_random_number() -> u32 {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..101);
    return random_number;
}
