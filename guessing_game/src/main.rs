use std::io;
use rand::Rng;

fn main() {

    //Entering Input here
    println!("Enter a String");
    //Mutable variable
    let mut guess = String::new();
    //Inputing the data
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to get String guess");
    //To check half part of the code
    println!("The no. {} is what you entered",guess);

}
