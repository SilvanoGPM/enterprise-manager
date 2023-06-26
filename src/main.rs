mod employee;
mod utils;

use crate::utils::{string, terminal};
use colored::*;
use std::collections::HashMap;

fn main() {
    const PARTS_IN_ADD_ACTION: usize = 7;
    const PARTS_IN_REMOVE_ACTION: usize = 5;

    let mut employees: employee::EmployeeDatabase = HashMap::new();

    loop {
        terminal::clear();

        let operation = terminal::read_string_no_empty("Command (Help to view all commands): ");

        let parts = operation
            .split_whitespace()
            .map(|x| string::title_case(x.trim()))
            .collect::<Vec<_>>();

        let action = &parts[0];

        let operation = string::title_case(&operation);

        terminal::clear();

        match (&operation[..], &action[..]) {
            ("List All Employees", _) => employee::list_all(&mut employees),
            (_, "Add") if parts.len() == PARTS_IN_ADD_ACTION => {
                let name = &parts[1];
                let deparment = &parts[3];
                let phone = &parts[6];

                employee::save(&mut employees, &name, &deparment, &phone);
            }
            (_, "Remove") if parts.len() == PARTS_IN_REMOVE_ACTION => {
                match parts[4].parse::<i32>() {
                    Ok(id) => employee::remove(&mut employees, id),
                    Err(_) => println!("{}", "Insert a valid id!".red()),
                };
            }
            ("Exit", _) => {
                println!("{}", "Leaving...".red());
                break;
            }
            ("Help", _) => {
                println!("{}", "Commands:".green());
                println!("{}", "List All Employees".cyan());
                println!("{}", "Add <NAME> To <DEPARMENT> With Phone <PHONE>".cyan());
                println!("{}", "Remove Employee With Id <ID>".cyan());
                println!("{}", "Exit".cyan());
            }
            _ => println!("{}", "This command dont exists.".red()),
        }

        terminal::press_any_key_to_continue();
    }

    println!("{}", "Application finished.".red());
}
