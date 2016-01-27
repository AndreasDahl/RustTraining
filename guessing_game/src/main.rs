extern crate rand;

use std::io;
use rand::random;
use std::cmp::Ordering;

fn cmp(a: u32, b: u32) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}

fn main() {
    println!("Guess the number!");

    let secret_number = (rand::random::<u32>() % 100) + 1;

//    println!("The secret number is: {}", secret_number);

    loop {

        println!("Please input your guess.");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input_num: Option<u32> = input.trim().parse().ok();
                let num = match input_num {
                    Some(num)   => num,
                    None        => {
                        println!("Please input a number!");
                        continue;
                    }
                };

                println!("You guessed: {}", input);

                match cmp(num, secret_number) {
                    Ordering::Less      => println!("Too small!"),
                    Ordering::Greater   => println!("Too big!"),
                    Ordering::Equal     => {
                        println!("You win!");
                        return
                    },
                }
            }
            Err(error) => println!("Failed to read line, try again: {}", error),
        }
    }
}
