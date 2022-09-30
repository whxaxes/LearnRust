use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..100);
    
    loop {
        println!("Please Input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please type a number");
                continue
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too large!"),
            Ordering::Equal => {
                println!("correct!");
                break;
            }
        }
    }
}
