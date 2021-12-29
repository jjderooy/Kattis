
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut time: Vec<i32> = Vec::new();
    let mut dist: Vec<i32> = Vec::new();

    let mut handle = stdin.lock().lines();
    handle.next(); // Skip first line since it is not data

    // loop through each string in stdin
    for line in &mut handle.map(|x| x.unwrap()){
        let mut iter = line.split_whitespace();

        // Extract the two integers from the string
        time.push(iter.next().unwrap().parse::<i32>().unwrap());
        dist.push(iter.next().unwrap().parse::<i32>().unwrap());
    }

    let mut normalized_time: Vec<i32> = Vec::new();
    let mut normalized_dist: Vec<i32> = Vec::new();

    // These will always be equal unless there is an error in the data.
    assert_eq!(time.len(), dist.len());

    if time.len() > 1{

        // Normalized the values so they are not relative to the prev val
        for i in 1..time.len(){
            normalized_time.push(time[i] - time[i-1]);
            normalized_dist.push(dist[i] - dist[i-1]);
        }

        // Compute and find maximum speed
        let mut max_speed: i32 = 0;

        for i in 0..normalized_time.len(){
            let speed = normalized_dist[i] / normalized_time[i];

            if max_speed < speed{
                max_speed = speed;
            }
        }

        println!("{}", max_speed);
    }
    else{
        // Input will always contain at least one line 0,0 so speed is 0
        print!("0");
    }

}