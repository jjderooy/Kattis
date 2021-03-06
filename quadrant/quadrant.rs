use std::io;

fn main(){
    // String to store the users input
    let mut x = String::new();
    let mut y = String::new();

    // Read a line from standard input to our string. Print an error if something went wrong
    io::stdin()
        .read_line(&mut x)
        .expect("Aw shit something went wrong.");

    std::io::stdin()
        .read_line(&mut y)
        .expect("Aw shit something went wrong.");  
    
    // Convert to an int. We use let to convert or "shadow" a to an int 32
    let x: i16 = x.trim().parse().expect("Should have been an int");
    let y: i16 = y.trim().parse().expect("Should have been an int");

    // Comprare to see what quadrant it lies in
    if x > 0 && y > 0{
        println!("1");
    }
    else if (x < 0 && y > 0){
        println!("2")
    }
    else if (x < 0 && y < 0){
        println!("3")
    }
    else{
        println!("4")
    }
}
