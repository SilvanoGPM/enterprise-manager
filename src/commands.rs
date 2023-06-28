use regex::Regex;

pub struct AddEmployeeProps {
    name: String,
    department: String,
    phone: String,
}

impl AddEmployeeProps {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn department(&self) -> &String {
        &self.department
    }

    pub fn phone(&self) -> &String {
        &self.phone
    }

    fn empty() -> Self {
        Self {
            name: String::new(),
            department: String::new(),
            phone: String::new(),
        }
    }
}

pub enum Command {
    ListAllEmployees,
    AddEmployee(AddEmployeeProps),
    RemoveEmployee(Option<i32>),
    Exit,
    Help,
}

pub fn get_command(command: &str) -> Option<Command> {
    let commands = [
        (
            Regex::new(r"(?i)List All Employees").unwrap(),
            Command::ListAllEmployees,
        ),
        (
            Regex::new(r"(?i)Add (\w+) To (\w+) With Phone (\w+)").unwrap(),
            Command::AddEmployee(AddEmployeeProps::empty()),
        ),
        (
            Regex::new(r"(?i)Remove Employee With id (\w+)").unwrap(),
            Command::RemoveEmployee(None),
        ),
        (Regex::new(r"(?i)Help").unwrap(), Command::Help),
        (Regex::new(r"(?i)Exit").unwrap(), Command::Exit),
    ];

    for (regex, command_type) in &commands {
        if let Some(captures) = regex.captures(command) {
            return match command_type {
                Command::ListAllEmployees => Some(Command::ListAllEmployees),
                Command::AddEmployee(_) => {
                    let employee = get_add_employee_props(&captures)?;
                    Some(Command::AddEmployee(employee))
                }
                Command::RemoveEmployee(_) => {
                    let id = get_remove_employee_id(&captures);
                    Some(Command::RemoveEmployee(id))
                }
                Command::Help => Some(Command::Help),
                Command::Exit => Some(Command::Exit),
            };
        }
    }

    None
}

fn get_add_employee_props(captures: &regex::Captures) -> Option<AddEmployeeProps> {
    let name = captures.get(1)?.as_str().to_string();
    let department = captures.get(2)?.as_str().to_string();
    let phone = captures.get(3)?.as_str().to_string();

    Some(AddEmployeeProps {
        name,
        department,
        phone,
    })
}

fn get_remove_employee_id(captures: &regex::Captures) -> Option<i32> {
    match captures.get(1)?.as_str().parse::<i32>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
