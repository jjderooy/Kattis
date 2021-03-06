use std::io;

fn main(){
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Oh no");
    
    // This returns an iterator
    let mut numbers = input.split_whitespace();
    let mut product: i32 = 1; 

    for num in numbers{
        product *= num.parse::<i32>().unwrap();
    }

    println!("{}", product);
}