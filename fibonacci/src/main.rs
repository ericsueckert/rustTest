use std::io;


fn main() {
    let mut reader = io::stdin();
    let mut input = String::new();

    println!("Which number of the fibonacci sequence would you like to calculate?");

    reader.read_line(&mut input).ok().expect("failed to read line");

    let input_opt: Option<u32> = input.trim().parse::<u32>().ok();

    let input_u = match input_opt {
        Some(input_u) => input_u,
        None => {
            println!("Please input a positive integer");
            return;
        }
    };

    println!("{}", fibonacci(input_u));
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
