mod repository;

use crate::utils::{string, terminal};
use colored::*;
use repository::Employee;
pub use repository::EmployeeDatabase;

pub fn list_all(employees_db: &EmployeeDatabase) {
    if employees_db.is_empty() {
        println!("{}", "No employees.".yellow());
        println!();
        return;
    }

    println!("{}", "Employees: ".green());
    println!();

    for (department, employees) in employees_db {
        if !employees.is_empty() {
            println!("{} {}", department.blue(), "Department:".blue());

            for employee in employees {
                println!("  Employee {}:", employee.name().green());

                if let Some(id) = employee.id() {
                    println!("    Id: {}", id.to_string().green());
                }

                println!("    Phone: {}", employee.phone().green());
                println!();
            }
        }
    }
}

pub fn save(employees: &mut EmployeeDatabase) {
    println!("{}", "Add new employee:".green());
    println!();

    let name = terminal::read_string_no_empty("Name: ");
    let department = terminal::read_string_no_empty("Department: ");
    let phone = terminal::read_string_no_empty("Phone: ");

    let mut employee = Employee::new(
        string::title_case(&name[..]),
        string::title_case(&department[..]),
        phone,
    );

    let employee = repository::save(employees, &mut employee);

    println!(
        "{} {} {}.",
        employee.name().green(),
        "was added to department".green(),
        employee.department().green(),
    );

    println!();
}

pub fn remove(employees: &mut EmployeeDatabase) {
    let id = terminal::read_number("Employee id (-1 to cancel): ");

    let is_canceled = id == -1;

    if is_canceled {
        println!("{}", "Removal canceled!".yellow());
        return;
    }

    let removed = repository::remove(employees, Some(id));

    if removed {
        println!(
            "{} {} {}",
            "Employee with id".green(),
            id.to_string().green(),
            "removed.".green()
        );
    } else {
        println!(
            "{} {} {}",
            "Employee with id".red(),
            id.to_string().red(),
            "not found.".red()
        );
    }
}
