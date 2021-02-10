// Crate = collection of Rust code
// This below is binary crate. Dependency is a library crate.

// import Rng trait with random number generator implementations to be able to use gen_range() below
use rand::Rng;
// bring io library (part of std library) into scope
use std::cmp::Ordering;
use std::io;

// keyword, entry point
fn main() {
    println!("Guess the number!");

    // thread_rng() - get particular random number generator local to current thread and seeded by OS
    // gen_range() inclusive on the lower bound and exclusive on the upper bound
    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("Secret: {}", secret_number);

    loop {
        println!("Enter your guess:");

        // create a mutable variable. Immutable by default. new() returns a new instance of String.
        // String is part of std library. Growable, utf-8 bit of text.
        // :: means it's "associated" (static) function of the String type, not of the instance
        // no need to specify type, String type of guess variable is inferred from String::new()
        let mut guess = String::new();

        // can write std::io::stdin(). Returns an instance of std::io::Stdin - standard terminal input
        let num_bytes = io::stdin()
            // & indicates argument is a reference. References are immutable by default, so adding mut
            // returns an instance of io::Result. Type is enumeration - fixed set of values (variants).
            // For Result, the variants are Ok or Err. Both contain generated value/error details
            .read_line(&mut guess)
            // if Err, expect() will crash with given message
            // if Ok, it will return result (here number of bytes read)
            .expect("Failed to read line");

        // "shadow" previous value of existing variable with a new one.
        // Shadowing allows to reuse variable name and not define a new one (guess_str, guess_num)
        // after input and pressing enter, guess will be "5\n". trim() removes whitespace.
        // .parse() can result in different number types. That's why annotating the variable type
        // match is a general way to handle error
        let guess: u8 = match guess.trim().parse() {
          // on successful parse, value will be put into Ok variant value
          Ok(num) => num,
          // _ is catchall value
          Err(_) => continue,
        };

        // {} is a placeholder
        println!(
            "You guessed: {}. Number of bytes read: {}",
            guess, num_bytes
        );

        // guess.cmp() returns a variant of Ordering enum type. 3 options listed below.
        // match expression is made of "arms" (outcomes)
        match guess.cmp(&secret_number) {
            // arm consists of pattern and code to run if it matches
            // each arm is checked for match. It stops on first match.
            Ordering::Less => println!("Try bigger"),
            Ordering::Greater => println!("Try smaller"),
            Ordering::Equal => {
              println!("Exactly!");
              break;
            }
        }
    }
}
