
// We have to use this rather than just io because .lines()
// is a nightly feature I think
use std::io::{self, BufRead};

fn main() {

    let stdin = io::stdin();

    // Get iterator over the lines in stdin 
    let mut lines_iter = stdin.lock().lines();

    // Risky since we don't know if there will be any "next"
    // We don't care about the first line of the file
    lines_iter.next();

    // .map applies the unwrap function to each element in the iterator
    for line in lines_iter.map(|x| x.unwrap()){
        
        let mut pb_iter = line.split_whitespace();

        // Risky since we don't know if there will be any "next"
        // Don't care about the first number
        pb_iter.next();
        
        let mut total_outlets: i32 = 1;

        for power_bar in pb_iter{
           total_outlets += power_bar
           .trim()
           .parse::<i32>()
           .expect("Not a number");

           // Each power bar uses 1 outlet
           total_outlets -= 1;
        }

        println!("{}", total_outlets);
    }

}