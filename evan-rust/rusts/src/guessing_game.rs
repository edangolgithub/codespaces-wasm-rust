use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn game_start() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        let guessn: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let ans = guessn.cmp(&secret_number);

        let message = match ans {
            Ordering::Less => "Too small!".to_string(),
            Ordering::Greater => "Too big!".to_string(),
            Ordering::Equal =>{
                println!("You win!");
                 break;
            },
        };
        println!("{}", message);
    }
}

pub fn load_game() {
    game_start();
}
