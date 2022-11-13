use std::io; // this allows us to use the io library (input/output)
// rust has a set of items defined in the standard library, called the prelude
// the prelude is a list of items that are imported into every module by default
// when theres is no prelude, we have to import using the use keyword
use rand::Rng; // this allows us to use the Rng trait from the rand library
use std::cmp::Ordering; // this allows us to use the Ordering enum from the cmp library

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please input your guess:");   

        let mut guess = String::new(); // :: means that new is an associated function of the String type
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // parse method converts a string to a number
        // parse returns a Result type, which is an enum that can be Ok or Err
        // the match expression is used to handle the Result type
        
        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
