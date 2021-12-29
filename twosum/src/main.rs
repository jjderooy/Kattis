
use std::io;

fn main() {

    // Get a string from standard input
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Unexpected input in stdin");

    // Get an iterator to each number in the string
    let split_iter = s.split_ascii_whitespace();

    let mut sum: i32 = 0;

    // Parse the numbers and sum them
    for num in split_iter{
        sum += num.trim().parse::<i32>().unwrap();
    }

    println!("{}", sum);
}
