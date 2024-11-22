use std::io::{stdin, stdout, Write};

use rand::{thread_rng, Rng};

fn main() {
    let mut is_playing = true;
    let n: u32 = thread_rng().gen_range(0..101);
    let mut guesses = 4;

    println!("Pick a number from 1 - 100. You have {} guesses", guesses);
    _ = stdout().flush();

    while is_playing {
        if guesses == 0 {
            println!("Correct guess was {}", n);
            break;
        }

        let mut a = String::new();
        stdin().read_line(&mut a).expect("Please provide a value");

        if a.trim().is_empty() {
            println!("No number provided");
            return;
        }

        if let Ok(an) = a.trim().parse::<u32>() {
            if an > 100 {
                println!("Please pick number between 1 and 100");
            }

            guesses -= 1;

            if an > n {
                println!("Too high! {} guesses left", guesses);
                continue;
            } else if an < n {
                println!("Too low! {} guesses left", guesses);
                continue;
            }

            if an == n {
                println!("YES YOU WON");
                is_playing = false;
            }
        } else {
            println!("Please enter number value");
            println!("Please pick number between 1 and 100");
        }
    }
}
