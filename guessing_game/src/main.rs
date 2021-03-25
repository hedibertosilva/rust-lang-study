use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", &secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim()
                          .parse()
                          .expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

/*
    Utils:

    @ Structure

            std::io::Result
            ^     ^       ^
        library::method::submethod

        io::stdin()
        ^        ^
        method::associated function

    @ Library Call

        `use std::io` can be optional that can be changed by:

        std::io::stdin()

        As well, we can use the word `as` to define the alias to the method.

    @ Variables

        By default, all the variables are created as immutable
        but we can change this using the work mut after let word.

        let name = 'Hediberto' // immutable
        let mut name = 'Hediberto' // mutable

    @ References

        &name  // immutable reference
        &mut name // mutable reference

*/
