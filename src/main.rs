use std::io::{self, BufRead};

fn main() {
    println!("Fizzbuzz, input number!");

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                if let Ok(number) = line.parse::<u32>() {
                    fizz_buzz(number)
                } else {
                    println!("Please input a number")
                }
            }
            _ => println!("Please input a number"),
        }
    }
}

fn fizz_buzz(number: u32) {
    for index in 0..number + 1 {
        if index % 3 == 0 && index % 5 == 0 {
            println!("Fizzbuzz")
        } else if index % 5 == 0 {
            println!("Buzz")
        } else if index % 3 == 0 {
            println!("Fizz")
        } else {
            println!("{}", index)
        }
    }
}
