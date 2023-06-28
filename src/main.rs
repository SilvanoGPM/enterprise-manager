mod commands;
mod employee;
mod utils;

use crate::commands::Command;
use crate::utils::{string, terminal};

use colored::*;
use commands::AddEmployeeProps;
use employee::EmployeeDatabase;
use std::collections::HashMap;

fn main() {
    let mut employees: employee::EmployeeDatabase = HashMap::new();

    loop {
        terminal::clear();

        let operation = terminal::read_string_no_empty("🤖 Command (Help to view all commands): ");
        let operation = string::title_case(&operation);

        terminal::clear();

        if let Some(command) = commands::get_command(&operation) {
            match command {
                Command::ListAllEmployees => employee::list_all(&mut employees),
                Command::AddEmployee(props) => add_employee(&mut employees, props),
                Command::RemoveEmployee(option) => remove_employee(&mut employees, option),
                Command::Help => help(),
                Command::Exit => break,
            }
        } else {
            println!("❌ {}", "This command dont exists.".red());
            println!("⚠️  {}", "Use help for a list of commands".blue());
            println!();
        }

        terminal::press_any_key_to_continue();
    }

    println!("🏁 {}", "Application finished.".red());
}

fn add_employee(employees: &mut EmployeeDatabase, props: AddEmployeeProps) {
    employee::save(employees, props.name(), props.department(), props.phone());
}

fn remove_employee(employees: &mut EmployeeDatabase, option: Option<i32>) {
    if let Some(id) = option {
        employee::remove(employees, id);
    } else {
        println!("❌ {}", "Insert a valid id!".red());
    }
}

fn help() {
    println!("{}", "Commands:".green());
    println!("{}", "List All Employees".cyan());
    println!("{}", "Add <NAME> To <DEPARMENT> With Phone <PHONE>".cyan());
    println!("{}", "Remove Employee With Id <ID>".cyan());
    println!("{}", "Exit".cyan());
}
