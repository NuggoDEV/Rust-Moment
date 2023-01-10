use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number from 1 - 101!");

    let secret_num = rand::thread_rng().gen_range(1, 101);
    println!("")

    println!("Please input your guess.");

    let mut u_guess = String::new();

    io::stdin()
        .read_line(&mut u_guess)
        .expect("Failed to read line!");

        let u_guess: u32 = u_guess.trim().parse().expect("Please type a number!");
    
    println!("You guessed: {}", u_guess);

    match u_guess.cmp(&secret_num)
    {
        Ordering::Less => println!("Incorrect! The answer was: {}", secret_num),
        Ordering::Greater => println!("Incorrect! The answer was: {}", secret_num),
        Ordering::Equal => println!("Correct answer!"),
    }

}
