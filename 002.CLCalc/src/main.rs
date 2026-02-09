use std::io::{self, Write};

fn get_number_from_stdin() -> Result<i64, std::io::Error> {
    let mut s = String::new();
    let result: i64;

    loop {
        print!("Please input an integer (64-bits, signed): ");
        io::stdout().flush().unwrap();
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
    println!("1. Plus");
    println!("2. Multiple");

    let choice: i64 = get_number_from_stdin()?;
    let x: i64 = get_number_from_stdin()?;
    let y: i64 = get_number_from_stdin()?;

    let mut result: i64 = 0;
    if choice == 1 {
        result = x + y;
    }
    else if choice == 2 {
        result = x * y;
    }

    println!("Result is {}", result);
    Ok(())
}
