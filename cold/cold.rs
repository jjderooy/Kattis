use std::io;

fn main(){
    let mut s = String::new();

    // We don't care about the first line
    io::stdin()
    .read_line(&mut s)
    .expect("Unexpected input");
    s.clear();

    // Store the second line in s
    io::stdin()
        .read_line(&mut s)
        .expect("Unexpected input");

    let mut days: u16 = 0;

    for temp in s.split_whitespace(){
        let num: i32 = temp.trim().parse().expect("Expected number");
        if num < 0{
            days += 1;
        }
    }

    println!("{}", days);
}