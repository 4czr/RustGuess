use std::cmp::Ordering;
use std::io;

fn main() {

    loop {
    
        println!("Welcome to this simple test RUST program!");
        
        println!("Please guess the number between 1-100");
        
        let mut guess = String::new();
        let act = rand::thread_rng().gen_range(1, 101);
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read");
            
        println!("You guessed: {}", guess);
        
        match guess.cmp(&act) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
