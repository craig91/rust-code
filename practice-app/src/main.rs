//use std::io;
// use rand::Rng;
fn main() {
    // println!("Guess the number!");
    // let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");
    // println!("Please input your guess.");
    // let mut guess = String::new();
    // io::stdin().read_line(&mut guess).expect("Failed to read line");
    // println!("You guessed: {guess}");


    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;

    println!("The value of y is {y}");

    let a: (i32, f64, u8) = (700, 60.4, 4);

    let six_hundred = a.0;
    let sixty_point_four = a.1;
    
    let four = a.2;

    println!("{six_hundred}, {sixty_point_four}, {four}")

}