use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use colored::*;

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn press_any_key_to_continue() {
    let mut value = String::new();

    println!("{}", "Press any key to continue...".yellow());

    io::stdin()
        .read_line(&mut value)
        .expect("Error on continue.");
}

pub fn read_string(msg: &str) -> String {
    print!("{}", msg.green());

    io::stdout().flush().expect("Error on flush.");

    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Error on read terminal input.");

    return value.trim().to_owned();
}

pub fn read_string_no_empty(msg: &str) -> String {
    loop {
        let value = read_string(msg);
        
        if value.is_empty() {
            continue;
        }

        return value;
    }
}

pub fn read_number<T>(msg: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    loop {
        let value = read_string(msg);

        let value: T = match value.trim().parse() {
            Ok(x) => x,
            Err(_) => continue,
        };

        return value;
    }
}
