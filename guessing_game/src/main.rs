use stdio::io; 

fn main() { // entry point into the program
    println!("Guess the number!");
    println!("Please input your guess");

    let mut guess = String::new();
    // variables in rust are immutable by default. In order to make it mutable you add the mut before the 
    // variable name.

    // "=" binds something to the variable.
    // on the right side of the "=" sign is the value that guess variable is bound to, which will be the 
    // calling of the String::new(), a function that returns a new instance of a String. *kind of seems like and object"
    // The :: syntax in the ::new line indicates that new is an associated function of the String type.
    // Ann associated function is a function that's implemented on a type, in this case String.
    // This "new" function creates a new, empty string.
    io::stdin()
        .read_line(&mut guess) // gets input from the user and stores the input in the guess variable
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}