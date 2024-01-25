use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the game: Guess the number.");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        println!("Type a number: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");
        
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                println!("You guessed: {guess}");
                num
            },
            Err(_) => {
                println!("Remember, only type numbers");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{guess} is too small \n"),
            Ordering::Greater => println!("{guess} is too big \n"),
            Ordering::Equal => { 
                println!("{guess} is the number! You win!");
                break;
            }
        }
    }

}
