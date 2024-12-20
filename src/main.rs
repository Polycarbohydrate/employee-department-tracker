use std::io;
use std::collections::HashMap;
fn main()   {
    let mut database: HashMap<String, String> =  HashMap::new();
    loop {
        println!("Enter the employee first name:");
        let mut first_name = String::new();
        io::stdin().read_line(&mut first_name).expect("Could not read line.");
        println!("Enter the employee last name:");
        let mut last_name = String::new();
        io::stdin().read_line(&mut last_name).expect("Could not read line.");
        println!("Enter the employee department:");
        let mut department = String::new();
        io::stdin().read_line(&mut department).expect("Could not read line.");
        let department = department.trim().replace("\n\r", "");
        let full_name = format!("{} {}", first_name.trim(), last_name.trim());
        database.insert(full_name, department);
        println!("Do you want to add another employee? (yes/no)");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Could not read line.");
        if answer.trim() == "no" {
            println!("Would you like to view the database? (yes/no)");
            let mut view_answer = String::new();
            io::stdin().read_line(&mut view_answer).expect("Could not read line.");
            if view_answer.trim().to_lowercase() == "yes" {
                let mut department_map: HashMap<String, Vec<String>> = HashMap::new();
                for (name, department) in &database {
                    department_map.entry(department.clone()).or_insert(Vec::new()).push(name.clone());
                }
                let mut departments: Vec<&String> = department_map.keys().collect();
                departments.sort();
                for department in departments {
                    println!("==============================");
                    println!("Department: {}", department);
                    let mut employees = department_map.get(department).unwrap().clone();
                    employees.sort();
                    for employee in employees {
                        println!("  - {}", employee);
                    }
                }
                println!("==============================");
                println!("Press enter to continue...");
                let mut pause = String::new();
                io::stdin().read_line(&mut pause).expect("Could not read line.");

                println!("Would you like to add more names? (yes/no)");
                let mut input_again = String::new();
                io::stdin().read_line(&mut input_again).expect("Could not read line.");
                if input_again.trim().to_lowercase() == "yes" {
                    continue
                }
                else if input_again.trim().to_lowercase() == "no" {
                    break
                }
                else {
                    println!("Please either type 'yes' or 'no'.");
                    continue
                }
            }
            else {
                break
            }
        }
        else if answer.trim().to_lowercase() == "yes" {
            continue
        }
        else {
            println!("Please either type 'yes' or 'no'.");
            continue
        }
    }
}