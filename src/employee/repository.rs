#[derive(Clone, Debug)]
pub struct Employee {
    id: Option<i32>,
    name: String,
    department: String,
    phone: String,
}

impl Employee {
    pub fn new(name: String, department: String, phone: String) -> Employee {
        Employee {
            id: None,
            name,
            department,
            phone,
        }
    }

    pub fn id(&self) -> &Option<i32> {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn department(&self) -> &String {
        &self.department
    }

    pub fn phone(&self) -> &String {
        &self.phone
    }
}

pub type EmployeeDatabase = HashMap<String, Vec<Employee>>;

use std::collections::HashMap;

pub fn save(employees: &mut EmployeeDatabase, employee: &mut Employee) -> Employee {    
    let deparments_length = employees.len() as i32;

    let department_employees = employees
        .entry(employee.department.to_owned())
        .or_insert(Vec::new());

    let employees_in_department = department_employees.len() as i32;

    employee.id = Some((deparments_length + 1) + (employees_in_department + 1));

    department_employees.push(employee.clone());

    employee.clone()
}

pub fn remove(employees: &mut EmployeeDatabase, id: Option<i32>) -> bool {
    let values = employees.values_mut();

    for department in values {
        let removed = department.iter()
            .filter(|x| x.id != id)
            .cloned()
            .collect::<Vec<_>>();

        if department.len() != removed.len() {
            *department = removed;
            return true;

        }
    }

    false
}
