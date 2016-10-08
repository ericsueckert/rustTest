use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    //mutable variable binding
    //:: refers to static method or object associated function

    //standard input function of io object
    //&mut reference mutable. Safe and easy to use references in Rust
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
