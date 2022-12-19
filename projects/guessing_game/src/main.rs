use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn enter_guess() -> u32 {
    let mut guess = String::new();
    println!("Please input your guess.");
    stdin().read_line(&mut guess).expect("Failed to read line.");
    match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => return enter_guess(),
    }
}

fn check_guess(secret_number: &u32, guess: u32) {
    match guess.cmp(secret_number) {
        Ordering::Equal => println!("You won!"),
        Ordering::Less => {
            println!("Guess bigger");
            check_guess(secret_number, enter_guess())
        }
        Ordering::Greater => {
            println!("Guess smaller");
            check_guess(secret_number, enter_guess())
        }
    }
}

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let guess = enter_guess();
    println!("You guessed: {guess}");
    check_guess(&secret_number, guess);
}
