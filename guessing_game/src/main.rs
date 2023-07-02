use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn check_guess(guess:i32, random_number:i32) -> bool {
    match guess.cmp(&random_number) {
        Ordering::Equal => {
            println!("You found the number!");
            return true; // i guess use return when early
        }
        Ordering::Greater => println!("Your guess was too high"),
        Ordering::Less => println!("Your guess was too low"),
    }
    false // this is the weirdest way to return 🗿
}

fn main() {
    let mut guess = 0;
    let random_number = rand::thread_rng().gen_range(0..100);
    // label the loop, not that important but can 
    'game: while guess != random_number {
        println!("enter a guess between 0 and 100:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
                   .expect("failed to read guess");
        guess = input.trim().parse::<i32>().expect("not a number");
        if check_guess(guess, random_number) {
            break 'game;
        }
    }
}
