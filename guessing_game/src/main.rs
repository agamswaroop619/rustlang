use std::io;

fn main() {
    println!("Enter a String");

    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to get String guess");

    println!("The no. {} is what you entered",guess);
       
}
