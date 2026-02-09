use std::io::{self, Write};

fn get_number_from_stdin() -> Result<i64, std::io::Error> {
    let mut s = String::new();
    let result: i64;

    loop {
        print!("Please input an integer (64-bits, signed): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut s)?;
        s = s.trim().to_string();
        match s.parse::<i64>() {
            Ok(res) => {
                result = res;
                break;
            },
            Err(_) => {
                println!("Failed to parse {} to i64.", s);
                println!("Please try again.");
                s = "".to_string();
            }
        }
    }
    Ok(result)
}

fn main() -> Result<(), std::io::Error> {
    println!("Please make a choice:");
    println!("\t1. Addition.");
    println!("\t2. Subtraction.");
    println!("\t3. Multiplication.");
    println!("\t4. Division.");

    let mut choice: i64;
    loop {
        choice = get_number_from_stdin()?;
        if 1 <= choice && choice <= 4 {
            break;
        }
        else {
            println!("Illegal choice: {}.", choice);
            println!("Please try again.")
        }
    }

    let x: i64 = get_number_from_stdin()?;
    let y: i64 = get_number_from_stdin()?;

    let result: i64;
    match choice {
        1 => result = x + y,
        2 => result = x - y,
        3 => result = x * y,
        4 => result = x / y,
        _ => result = 0
    }

    println!("Result is {}.", result);
    Ok(())
}
