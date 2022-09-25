use std::io;
// 'std' stands for the standard library
// 'io' stands for the input/output library (module)
// 'use' brings the input/output library into scope (it isn't in the prelude)

fn main() {
// the entry point into the program ('fn' stands for function)

    println!("Guess the number!");
    // 'println!' is a macro that prints a string to the screen

    println!("Please input your guess.");

    let mut guess = String::new();
    // 'mut' makes a variable mutable ('let mut' is equivalent to 'let' in JS)
    // String is a type provided by the standard library (prelude?)
    // 'new' returns a new string instance
    // :: indicates that 'new' is an associated function of String

    //let apples = 5; // immutable
    // prefix a variable with _ to disable the unused variable warning

    // Rust's 'let' is equivalent to JS's 'const' (the value is immutable)
    //let mut bananas = 5; // mutable

    io::stdin() // call the 'stdin' func of the 'io' module
    //std::io::stdio // no need the import 'io'
        .read_line(&mut guess) // call the read_line method
        // & indicates that the argument is a reference
        // &guess would be an immutable reference
        // we need a mutable one, so we're using '&mut guess'

        .expect("Failed to read line");
        // read_line returns a Result enum (Ok or Err)
        // this enum has an 'expect' method
        // if the value is Err, the string passed to 'expect' will be printed
        // if it's Ok, expect returns the byte number

    //io:stdin().read_line(&mut guess).expect("Failed to read line");
    // more difficult to read, but would work too

    println!("You guessed: {guess}"); // {guess} is equivalent to ${guess} in JS
    //println!("You guessed: {guess}{apples}");
    //println!("You guessed: {}{}", guess, apples);
}
