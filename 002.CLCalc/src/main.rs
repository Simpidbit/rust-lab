use std::io::{self, Write};

enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

struct Calculator {
    operator: Operator,
    opnd1: i64,
    opnd2: i64,
}

impl Calculator {
    fn new(operator: Operator, opnd1: i64, opnd2: i64) -> Self {
        Calculator { operator, opnd1, opnd2 }
    }

    pub fn get_result(&self) -> Option<i64> {
        match self.operator {
            Operator::Addition => {
                Some(self.opnd1 + self.opnd2)
            },
            Operator::Subtraction => {
                Some(self.opnd1 - self.opnd2)
            },
            Operator::Multiplication => {
                Some(self.opnd1 * self.opnd2)
            },
            Operator::Division => {
                if self.opnd2 == 0 {
                    None
                }
                else {
                    Some(self.opnd1 / self.opnd2)
                }
            }
        }
    }
}

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

    let calculator = Calculator::new(match choice {
        1 => Operator::Addition,
        2 => Operator::Subtraction,
        3 => Operator::Multiplication,
        4 => Operator::Division,
        _ => unreachable!("Choice limited to 1~4 by previous loop."),
    }, get_number_from_stdin()?, get_number_from_stdin()?);

    match calculator.get_result() {
        Some(val) => 
            println!("Result is {}.", val),
        None =>
            println!("Divisor is 0!"),
    }

    Ok(())
}
