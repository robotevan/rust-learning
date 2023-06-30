use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut guess = 0;
    let random_number = rand::thread_rng().gen_range(0..100);

    'game: while guess != random_number {
        println!("enter a guess between 0 and 100:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
                   .expect("failed to read guess");
        guess = input.trim().parse::<i32>().expect("not a number");
        match guess.cmp(&random_number) {
            Ordering::Equal => {
                println!("You found the number!");
                break 'game;
            }
            Ordering::Greater => println!("Your guess was too high"),
            Ordering::Less => println!("Your guess was too low"),
        }
    }
}
