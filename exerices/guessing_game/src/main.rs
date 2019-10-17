use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Generated number: {}", secret_number);

    loop {
        println!("Input your guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(value) => value,
            Err(msg) => {
                println!("Invalid number: {}", guess);
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("Yo winer");
                break;
            }
        }
    }
}
