use std::io;

fn get_number_from_stdin() -> Result<i64, ()> {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s).unwrap();
    Ok(s.trim().parse().unwrap())
}

fn main() {
    println!("Please make a choice:");
    println!("1. Plus");
    println!("2. Multiple");

    let choice: i64 = get_number_from_stdin().unwrap();

    let mut s = String::new();
    s = "".to_string();
    io::stdin()
        .read_line(&mut s).unwrap();
    let x: i64 = s.trim().parse().unwrap();

    s = "".to_string();
    io::stdin()
        .read_line(&mut s).unwrap();
    let y: i64 = s.trim().parse().unwrap();

    let mut result: i64 = 0;
    if choice == 1 {
        result = x + y;
    }
    else if choice == 2 {
        result = x * y;
    }

    println!("Result is {}", result);
}
