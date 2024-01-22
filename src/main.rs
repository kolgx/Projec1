use std::thread;
use std::time::Duration;

mod model {
    pub mod guess_number_game;
    pub mod tcp_communication_demo;
}

fn main() {
    println!("Hello, world!");
    let test_case = 1;
    match test_case {
        1 => {
            println!("test_case: {}", test_case);
            thread::spawn(|| {
                thread::sleep(Duration::from_secs(1));
                model::tcp_communication_demo::tcp_communication_demo_model::tcp_client_start();
            });
            model::tcp_communication_demo::tcp_communication_demo_model::tcp_server_start();
        }
        2 => {
            println!("test_case: {}", test_case);
            thread::spawn(|| {
                thread::sleep(Duration::from_secs(1));
                model::tcp_communication_demo::tcp_communication_demo_model::tcp_client_start();
            });
        }
        3 => {
            println!("test_case: {}", test_case);
            model::guess_number_game::guess_number_game_model::guess_number_game_start();
        }
        _ => {
            println!("test_case: {}", test_case);
            println!("Hello, world!");
        }
    }
}
