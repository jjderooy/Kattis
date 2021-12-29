
use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut chars_itr = input.chars();
     
    /* 
        This also works:
        if chars_itr.next() == Some('5'){
    */

    let mut is_prefix: bool = true;

    for _n in 0..3{
        if chars_itr.next().unwrap() != '5'{
            is_prefix = false;
        }
    }

    println!("{}", is_prefix as i16);
}
