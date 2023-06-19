use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // io::stdin().read_line(&mut number).expect("Number must between 1-100");

    // println!("You guessed: {number}");

    loop {
        println!("Please enter a number between 1-100: ");


        let mut number: String = String::new();
        let mut guess: Guess = Guess::new(io::stdin().read_line(&mut number).unwrap() as i32);
        // parsing number from string, assigning to guess
        let guess: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("number value: {number}");
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

struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Value must be between 1 and 100!");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

