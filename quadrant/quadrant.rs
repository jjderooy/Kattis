
fn main(){
    // String to store the users input
    let mut a = String::new();  

    // Read a line from standard input to our string. Print an error if something went wrong
    std::io::stdin()
        .read_line(&mut a)
        .expect("Aw shit something went wrong.");  
    
    // Convert to an int. We use let to convert or "shadow" a to an int 32
    let a: i16 = a.trim().parse().expect("Should have been an int");

    match a.std::cmp::Ordering(&1000){
        std::cpm::Ordering::Less => println!("Less than 1000"),
        std::cpm::Ordering::Greater => println!("Greater than 1000"),
        std::cpm::Ordering::Equal => println!("Equal to 1000"),
    }
}
