mod employee;
mod utils;

use std::collections::HashMap;
use colored::*;
use crate::utils::terminal;

fn main() {
    let mut employees: employee::EmployeeDatabase = HashMap::new();

    loop {
        terminal::clear();

        println!("{}", "--------------------------".blue());
        println!("{}", "|   ENTERPRISE MANAGER   |".blue());
        println!("{}", "--------------------------".blue());
        println!("{}", "| 1) List all employees  |".blue());
        println!("{}", "| 2) Add new employee    |".blue());
        println!("{}", "| 3) Remove employee     |".blue());
        println!("{}", "| 4) Exit                |".blue());
        println!("{}", "--------------------------".blue());

        let choose = terminal::read_string_no_empty("Choose: ");

        terminal::clear();

        match &choose[..] {
            "1" => employee::list_all(&mut employees),
            "2" => employee::save(&mut employees),
            "3" => employee::remove(&mut employees),
            "4" => {
                println!("{}", "Leaving...".red());
                break;
            }
            _ => println!("{}", "This option dont exists.".red()),
        }

        terminal::press_any_key_to_continue();
    }

    println!("{}", "Application finished.".red());
}
