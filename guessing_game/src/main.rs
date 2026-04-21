use std::{cmp::Ordering, io};

use rand::Rng;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}");
        }

        Guess {value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");

    
    loop {
        println!("Input your guess!");
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("Needs to be between 1 and 100");
            continue;
        }

        // let fib_n = fib(guess);
        // println!("The fibonacci number {fib_n}");
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Winner");
                break;
            }
        }    
    }


}

fn fib(n: u32) -> u32 {
    println!("{n}");
    if n == 0 || n == 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    let user1 = User {
        active: true,
        username: String::from("myuser"),
        email: String::from("saad@tajwar.com"),
        sign_in_count: 1
    };

    let user2 = User {
        email: String::from("abc"),
        ..user1
    };

    let black = Color(0, 0, 0);

    let subject = AlwaysEqual;

    &s[..]

}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

struct Rectangle {
    width: u32,
    height: u32,
}

fn my_func() {
    let x = Rectangle::square(3);
    let y = IpAddrKing::V4(String::from("abc"));

}

fn value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => 3,
    }
}

fn plus(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn another_plus(x: Option<i32>) {
    if let Some(number) = x {
        println!("lol {number}");
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

enum IpAddrKing {
    V4(String),
    V6(String),
}

enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}