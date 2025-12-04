use rand::Rng;
use regex::Regex;
use std::cmp::Ordering;
use std::io;

fn main() {
    celsius_fahrenheit_converter()
}

fn read_input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("read error");
    s
}

fn celsius_fahrenheit_converter() {
    println!("Enter the value you want to convert (i.e. \"12C\" or \"75F\"):");

    let re = Regex::new(r"(?<temp>-?\d+(?:\.\d+)?)(?<unit>[CF])").unwrap();
    loop {
        let input = read_input().trim().to_uppercase().to_string();

        let Some(caps) = re.captures(&input) else {
            println!("Invalid input, try again:");
            continue;
        };

        let temp = match caps["temp"].parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, try again:");
                continue;
            }
        };

        let unit = &caps["unit"];

        let (value, out_name) = match unit {
            "C" => (c_to_f(temp), "Fahrenheit"),
            "F" => (f_to_c(temp), "Celsius"),
            _ => unreachable!(),
        };

        println!("{input} is \x1b[1m{value:.2}° {out_name}\x1b[0m");
    }
}

fn c_to_f(n: f64) -> f64 {
    n * (9. / 5.) + 32.
}

fn f_to_c(n: f64) -> f64 {
    (n - 32.) * (5. / 9.)
}

fn guessing_game() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
