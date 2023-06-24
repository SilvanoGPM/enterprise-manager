mod employee;
mod utils;

use std::collections::HashMap;
use crate::utils::terminal;

fn main() {
    let mut employees: employee::EmployeeDatabase = HashMap::new();

    loop {
        terminal::clear();

        println!("--------------------------");
        println!("|   ENTERPRISE MANAGER   |");
        println!("--------------------------");
        println!("| 1) List all employees  |");
        println!("| 2) Add new employee    |");
        println!("| 3) Remove employee     |");
        println!("| 4) Exit                |");
        println!("--------------------------");

        let choose = terminal::read_string_no_empty("Choose: ");

        terminal::clear();

        match &choose[..] {
            "1" => employee::list_all(&mut employees),
            "2" => employee::save(&mut employees),
            "3" => employee::remove(&mut employees),
            "4" => {
                println!("Leaving...");
                break;
            }
            _ => println!("This option dont exists."),
        }

        terminal::press_any_key_to_continue();
    }

    println!("Application finished.");
}
