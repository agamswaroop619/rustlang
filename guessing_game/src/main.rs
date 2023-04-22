use rand::Rng;
use std::io;

fn main() {
    //Secret No.
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The no. {} is what we entered", secret_number);
    //Entering Input here
    println!("Enter a String");

    loop {
        let mut guess = String::new();
        //Inputing the data
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get String guess");
        //String to integer
        let guess: u32 = guess.trim().parse().expect("Please type a no.");
        println!("The no. {} is what you entered", guess);

        if guess > secret_number {
            println!("{} is larger than no. we predicted", guess);
        } else if guess < secret_number {
            println!("{} is smaller than no. we predicted", guess);
        } else {
            println!("You found the correct no.");
            break;
        }
    }
}
