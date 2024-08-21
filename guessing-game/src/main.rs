use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
  println!("Welcome to the guessing game!");

  let secret_number = rand::thread_rng().gen_range(1..=100);

  // println!("The secret number is {secret_number}");

  loop {
    println!("Enter your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line.");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    // match guess {
    //   Ok(v) => println!("{v}"),
    //   Err(e) => println!("error: {e}"),
    // }

    println!("Your guess: {guess}");

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      },
    }
  }

}