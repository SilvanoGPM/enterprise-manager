mod repository;

use crate::utils::string;
use colored::*;
use repository::Employee;
pub use repository::EmployeeDatabase;

pub fn list_all(employees_db: &EmployeeDatabase) {
    if employees_db.is_empty() {
        println!("🧑‍💼 {}", "No employees.".yellow());
        println!();
        return;
    }

    println!("{}", "🧑‍💼 Employees: ".green());
    println!();

    for (department, employees) in employees_db {
        if !employees.is_empty() {
            println!("🏬 {} {}", department.blue(), "Department:".blue());

            for employee in employees {
                println!("  👩‍⚕️  Employee {}:", employee.name().green());

                if let Some(id) = employee.id() {
                    println!("   🆔 Id: {}", id.to_string().green());
                }

                println!("   📱 Phone: {}", employee.phone().green());
                println!();
            }
        }
    }
}

pub fn save(employees: &mut EmployeeDatabase, name: &String, department: &String, phone: &String) {
    let mut employee = Employee::new(
        string::title_case(&name[..]),
        string::title_case(&department[..]),
        phone.clone(),
    );

    let employee = repository::save(employees, &mut employee);

    println!(
        "🆕 {} {} {}.",
        employee.name().green(),
        "was added to department".green(),
        employee.department().green(),
    );

    println!();
}

pub fn remove(employees: &mut EmployeeDatabase, id: i32) {
    let removed = repository::remove(employees, Some(id));

    if removed {
        println!(
            "🗑️ {} {} {}",
            "Employee with id".green(),
            id.to_string().green(),
            "removed.".green()
        );
    } else {
        println!(
            "🔎 {} {} {}",
            "Employee with id".red(),
            id.to_string().red(),
            "not found.".red()
        );
    }
}
