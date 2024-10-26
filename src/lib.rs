use sysinfo::{System, SystemExt, ProcessExt};
use std::time::Instant;

// Struct to represent an employee
pub struct Employee {
    pub employee_id: u32,
    pub age: u32,
    pub salary: u32,
    pub department: String,
    pub years_of_experience: u32,
}

// Function to create dummy data
pub fn dummy_df() -> (Vec<Employee>, f64, f64, u32, usize) {
    let departments = ["HR", "Finance", "IT", "Marketing", "Operations"];
    
    let employees = vec![
        Employee { employee_id: 1001, age: 25, salary: 50000, department: departments[0].to_string(), years_of_experience: 5 },
        Employee { employee_id: 1002, age: 30, salary: 60000, department: departments[1].to_string(), years_of_experience: 10 },
        Employee { employee_id: 1003, age: 35, salary: 70000, department: departments[2].to_string(), years_of_experience: 15 },
        Employee { employee_id: 1004, age: 40, salary: 80000, department: departments[3].to_string(), years_of_experience: 20 },
        Employee { employee_id: 1005, age: 28, salary: 55000, department: departments[4].to_string(), years_of_experience: 7 },
        Employee { employee_id: 1006, age: 45, salary: 90000, department: departments[1].to_string(), years_of_experience: 25 },
        Employee { employee_id: 1007, age: 32, salary: 65000, department: departments[2].to_string(), years_of_experience: 12 },
        Employee { employee_id: 1008, age: 29, salary: 62000, department: departments[0].to_string(), years_of_experience: 8 },
        Employee { employee_id: 1009, age: 31, salary: 58000, department: departments[3].to_string(), years_of_experience: 9 },
        Employee { employee_id: 1010, age: 34, salary: 76000, department: departments[4].to_string(), years_of_experience: 14 },
        Employee { employee_id: 1011, age: 27, salary: 53000, department: departments[1].to_string(), years_of_experience: 6 },
        Employee { employee_id: 1012, age: 26, salary: 54000, department: departments[2].to_string(), years_of_experience: 4 },
        Employee { employee_id: 1013, age: 33, salary: 63000, department: departments[0].to_string(), years_of_experience: 11 },
        Employee { employee_id: 1014, age: 38, salary: 85000, department: departments[3].to_string(), years_of_experience: 18 },
        Employee { employee_id: 1015, age: 36, salary: 72000, department: departments[4].to_string(), years_of_experience: 16 },
        Employee { employee_id: 1016, age: 37, salary: 71000, department: departments[1].to_string(), years_of_experience: 17 },
        Employee { employee_id: 1017, age: 39, salary: 78000, department: departments[2].to_string(), years_of_experience: 19 },
        Employee { employee_id: 1018, age: 42, salary: 83000, department: departments[0].to_string(), years_of_experience: 22 },
        Employee { employee_id: 1019, age: 41, salary: 82000, department: departments[3].to_string(), years_of_experience: 21 },
        Employee { employee_id: 1020, age: 43, salary: 87000, department: departments[4].to_string(), years_of_experience: 23 },
        Employee { employee_id: 1021, age: 44, salary: 88000, department: departments[1].to_string(), years_of_experience: 24 },
        Employee { employee_id: 1022, age: 46, salary: 91000, department: departments[2].to_string(), years_of_experience: 26 },
        Employee { employee_id: 1023, age: 48, salary: 95000, department: departments[0].to_string(), years_of_experience: 28 },
        Employee { employee_id: 1024, age: 49, salary: 98000, department: departments[3].to_string(), years_of_experience: 29 },
        Employee { employee_id: 1025, age: 50, salary: 100000, department: departments[4].to_string(), years_of_experience: 30 },
    ];
    
    let salaries: Vec<u32> = employees.iter().map(|e| e.salary).collect();
    let mean_salary = salaries.iter().sum::<u32>() as f64 / salaries.len() as f64;
    let median_salary = {
        let mut sorted_salaries = salaries.clone();
        sorted_salaries.sort();
        if sorted_salaries.len() % 2 == 0 {
            (sorted_salaries[sorted_salaries.len() / 2 - 1] as f64 + sorted_salaries[sorted_salaries.len() / 2] as f64) / 2.0
        } else {
            sorted_salaries[sorted_salaries.len() / 2] as f64
        }
    };
    let sum_salary = salaries.iter().sum();
    let count_salary = salaries.len();

    (employees, mean_salary, median_salary, sum_salary, count_salary)
}

// Function to get memory usage in MB
pub fn get_memory_usage() -> f64 {
    let s = System::new_all();
    let process = s.process(sysinfo::get_current_pid().expect("Failed to get current PID")).expect("Failed to get process");
    process.memory() as f64 / 1024.0  // Convert to MB
}

// Dummy processing function for performance measurement
pub fn process_data(data: &[u32]) -> u32 {
    data.iter().sum()
}
