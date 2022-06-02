use rand::Rng;
use std::io::stdin;
fn main() {
    println!("Beep boop, tell me a number, beep boop!");
    let mut guess = String::new();
    let f = stdin().read_line(&mut guess);
    let f = match f {
        Ok(n) => n,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    // println!("You guessed: {}", guess);
    let guess_number = guess.trim().parse::<u8>();
    let guess_number = match guess_number {
        Ok(n) => n,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    println!("You guessed: {:08b}", guess_number);
}
