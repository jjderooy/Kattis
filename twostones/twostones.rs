use std::io;

fn main(){
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Invalid input");

    let mut n: i32 = n.trim().parse().expect("Non integer value entered");

    if n % 2 == 0{
        println!("Bob");
    }
    else{
        println!("Alice");
    }
}
