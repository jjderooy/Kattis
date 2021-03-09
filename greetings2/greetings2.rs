use std::io;

fn main(){
    
    let mut s = String::new();

    io::stdin()
        .read_line(&mut s)
        .expect("Unexpected input");
    
    // Get total occurences of letter e
    let mut total_e: i16 = 0;
    for c in s.chars(){
        if c == 'e'{
            total_e += 1;
        }
    }

    // Building our string
    let mut hey = String::new();

    hey.push('h');

    // The underscore in front of i prevents error because we don't use i
    for _i in 0..(total_e)*2{
        hey.push('e');
    }
    hey.push('y');

    println!("{}", hey);
}