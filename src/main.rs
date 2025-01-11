use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the Number!");
    loop {
    
    let secrate_number = rand::thread_rng().gen_range(1..=100); // Start..=End
        println!("The Secreate Number is {secrate_number}");
        println!("Please Input your Guess.");
    
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed: {guess}");

        match guess.cmp(&secrate_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!();
    }
        
} 
