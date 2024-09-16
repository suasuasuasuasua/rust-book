// Use the std::io library to read input from the console
// The `use` qualifier allows us to say io::stdin instead of std::io::stdin for
// shorthand
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Generate the secret random number
    let lower_bound = 1;
    let upper_bound = 100;
    let secret_number = rand::thread_rng().gen_range(lower_bound..=upper_bound);
    // Only print the secret number in debug mode
    if cfg!(debug_assertions) {
        println!("The secret number is {}", secret_number);
    }

    loop {
        println!("Please input your guess.");

        // `guess` is where we store the user's input
        // The reason why it is marked `mut` is because the user will change it
        // via input.
        // If this qualifier was left off, then the variable would be immutable
        let mut guess = String::new();

        io::stdin()
            // Call the IO function to read from stdin (i.e. the console) and append
            // it to the string
            .read_line(&mut guess)
            // If we encounter some error, then throw and print out some message
            // Note that since read_line() returns a Result Enum, we need to handle
            // it in some way or else the program is *unsafe*
            .expect("Failed to read line");

        // Cast the user's guess to an integer so that we can compare it to the
        // secret number -- note that `guess` shadowing on the previous definition
        //
        // We can match guess (after it's been trimmed and parsed) to its possible
        // Result types
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Could not parse the number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Just right!");
                break;
            }
        }
    }
}
