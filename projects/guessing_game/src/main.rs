use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::rng().random_range(1..=100);
    loop {
        println!("Guess the number!");
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line")
        ;

        // if Return is 'Ok', return the guess
        // if Return is 'Err', return continue
            // 'continue' makes the loop go to the next iteration
        // '_' is for catchall, so ALL error values count for this statement
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        };

        // let guess replaces the previous guess variable with a u32 version of
            // it
        // trim() gets rid of the \n appended when you type enter to input the
            // data
        // parse() is used to parse the string and return 
        // not everything can be turned into a u32int, so a Result will be
            // returned, which is why we have .expect() NOT ANYMORE THO

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
        println!()
    }
}
