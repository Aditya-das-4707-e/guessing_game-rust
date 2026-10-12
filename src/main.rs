use rand::RngExt;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");
    let secret_number = rand::rng().random_range(1..101);
    // println!("The secret number is: {}", secret_number);

    // println!("Please input your guess : ");
    loop {
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("code run failed");

        let guess: u32 = guess.trim().parse().expect("code run failed");

        // println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
