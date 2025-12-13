use std::io;

fn game_start() {
    println!("Guess a number");
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    io::stdin().read_line(&mut guess).expect("cannot input");

    println!("{}", guess);
}

pub fn load_game() {
    game_start();
}
