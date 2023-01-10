use std::io;
use std::cmp::Ordering;

use rand::Rng;

use colored::*;

fn main() {
    println!("Guess the number from 1 - 101!");

    let secret_num= rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut u_guess = String::new();
    
        io::stdin()
            .read_line(&mut u_guess)
            .expect("Failed to read line!");
    
            let u_guess: u32 = match u_guess.trim()
                .parse()
                {
                    Ok(num) => num,
                    Err(_) => continue,
                };
        
        println!("You guessed: {}", u_guess);
    
        match u_guess.cmp(&secret_num)
        {
            Ordering::Less => println!("{}", "The answer is higher!".red()),
            Ordering::Greater => println!("{}", "The answer is lower!".red()),
            Ordering::Equal => {
                println!("{}", "Correct answer!".green());
                break;
            },
        }      
        
    }

}
