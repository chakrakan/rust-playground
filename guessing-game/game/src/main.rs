// from stdlib, input output import the IO module
use std::io::{self, Write};
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    // 1 to 101 inclusive lower exclusive upper bound
    let secret_number = rand::thread_rng().gen_range(1, 11);

    // macro to print line
    println!("Guess the number!");

    // loop keyword creates an infinite loop
    loop {
        print!("Please input your guess from 1-10: ");
        io::stdout().flush().expect("Flush");
    
        // let is used to declare a variable. In Rust, variables are immutable by default, therefore adding mut before
        // enables this variable to be mutable.
        // Here, String::new() is a function that returns a new instance of a String, a growable type which is utf-8 encoded bit of text.
        // :: syntax indicates that new is an associated function of the String type. 
        // An associated function is implemented on a type, in this case String, rather than on a particular instance of a String. Some languages call this a static method.
        let mut guess = String::new();
    
        // call stdin() function from the io module
        // The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
        io::stdin()
            // The next part of the code, .read_line(&mut guess), calls the read_line method on the standard input handle to get input from the user. 
            // We’re also passing one argument to read_line: &mut guess.
            // The job of read_line is to take whatever the user types into standard input and place that into a string, so it takes that string as an argument. 
            // The string argument needs to be mutable so the method can change the string’s content by adding the user input.
            // The `&` indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
            .read_line(&mut guess)
            // read_line puts what the user types into the string we’re passing it, but it also returns a value—in this case, an io::Result. 
            // Rust has a number of types named Result in its standard library: a generic Result as well as specific versions for submodules, such as io::Result
            // The Result types are enumerations, often referred to as enums. 
            // An enumeration is a type that can have a fixed set of values, and those values are called the enum’s variants
    
            //For Result, the variants are Ok or Err. The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value. 
            // The Err variant means the operation failed, and Err contains information about how or why the operation failed
            // An instance of io::Result has an expect method that you can call. 
            // If this instance of io::Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect
            .expect("Failed to read line");
    
            // NOTE: If you don’t call expect, the program will compile, but you’ll get a warning
    
        // convert to unsigned 32 bit integer by "shadowing" previous value of guess with new one
        // trim eliminates any whitespace and trailing new lines since input would look like `5\n` for e.g. Then parse will parse to u32
        let guess: u32 = match guess
            .trim()
            // switching from expect to match allows us to handle the error and continue onwards
            // if parse works, it'll return an Ok value, if not it'll throw an Err
            // the '_' is a catchall for errors
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
               
        // printing with placeholders        
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess was lower!"),
            Ordering::Greater => println!("Your guess was higher!"),
            Ordering::Equal => {
                println!("You guessed it right! The number was indeed {}, You win!", secret_number);
                break;
            }
        }        
    }
}