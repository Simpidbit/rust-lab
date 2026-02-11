use std::io::{self, Write};

fn get_number_from_stdin() -> Result<i64, std::io::Error> {
    loop {
        let mut s = String::new();
        print!("Please input an integer (64-bits, signed): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut s)?;
        match s.trim().parse::<i64>() {
            Ok(res) => break Ok(res),
            Err(_) => {
                println!("Failed to parse {} to i64.", s.trim());
                println!("Please try again.");
            }
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    println!("Please make a choice:");
    println!("\t1. Addition.");
    println!("\t2. Subtraction.");
    println!("\t3. Multiplication.");
    println!("\t4. Division.");

    let choice: i64 = loop {
        let input_choice = get_number_from_stdin()?;
        if (1..=4).contains(&input_choice) {
            break input_choice;
        }
        else {
            println!("Illegal choice: {}.", input_choice);
            println!("Please try again.");
        }
    };

    let x: i64 = get_number_from_stdin()?;
    let y: i64 = get_number_from_stdin()?;

    let result: Option<i64> = match choice {
        1 => Some(x + y),
        2 => Some(x - y),
        3 => Some(x * y),
        4 => {
            if y == 0 {
                None
            }
            else {
                Some(x / y)
            }
        },
        _ => unreachable!("Choice limited to 1~4 by previous loop."),
    };

    match result {
        Some(val) => 
            println!("Result is {}.", val),
        None =>
            println!("Divisor is 0!"),
    }

    Ok(())
}
