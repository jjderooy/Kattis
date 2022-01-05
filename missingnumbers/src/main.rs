
use std::io::{self, BufRead};

fn main() {

    let stdin = io::stdin();
    let mut handle = stdin.lock().lines();

    // Disregard first line of input
    handle.next();

    let mut n = 1;
    let mut no_missing = true;

    for line in handle.map(|x| x.unwrap().trim().parse::<u8>().unwrap()){

        // Print all the numbers missing.
        for i in n..line{
            println!("{}", i);
            n += 1;
            no_missing = false;
        }

        n += 1;
    }

    // No missing numbers
    if no_missing{
        println!("good job");
    }
}
