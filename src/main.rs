// mod model {
//     pub mod test_demo;
// }

use rand::Rng;
// use crate::model::test_demo::test_demo::test_demo;

fn main() {
    let guess = create_random_number();
    println!("The secret number is: {}", guess);

    loop {
        let input : u32 = get_user_input();
        print!("your input value is: {}, ", input);

        match input.cmp(&guess) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

fn get_user_input() -> u32 {
    let mut input = String::new();
    loop {
        input.clear();
        println!("Please input your number:");
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input_int: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number {}, please input again", input.trim());
                continue;
            },
        };
        return input_int;
    }
}

fn create_random_number() -> u32 {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..101);
    return random_number;
}
