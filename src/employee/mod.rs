mod repository;

use crate::utils::{string, terminal};

use repository::Employee;
pub use repository::EmployeeDatabase;

pub fn list_all(employees_db: &EmployeeDatabase) {
    if employees_db.is_empty() {
        println!("No employees.");
        println!();
        return;
    }

    println!("Employees: ");
    println!();

    for (department, employees) in employees_db {
        if !employees.is_empty() {
            println!("{department} Department:");

            for employee in employees {
                println!("  Employee {}:", employee.name());

                if let Some(id) = employee.id() {
                    println!("    Id: {}", id);
                }

                println!("    Phone: {}", employee.phone());
                println!();
            }
        }
    }
}

pub fn save(employees: &mut EmployeeDatabase) {
    println!("Add new employee:");
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

    let mut id = 0;

    if let Some(x) = employee.id() {
        id = *x;
    }

    println!(
        "{} was added to department {} with id \"{}\".",
        employee.name(),
        employee.department(),
        id
    );

    println!();
}

pub fn remove(employees: &mut EmployeeDatabase) {
    let id = terminal::read_number("Employee id (-1 to cancel): ");

    let is_canceled = id == -1;

    if is_canceled {
        println!("Removal canceled!");
        return;
    }

    let removed = repository::remove(employees, Some(id));

    if removed {
        println!("Employee with id \"{id}\" removed");
    } else {
        println!("Employee with id \"{id}\" not found");
    }
}
