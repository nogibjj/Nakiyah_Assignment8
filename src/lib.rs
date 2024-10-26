use rand::Rng;
use rand::distributions::{Uniform, Slice};
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
    let mut rng = rand::thread_rng();
    let departments = ["HR", "Finance", "IT", "Marketing", "Operations"];
    
    let employees: Vec<Employee> = (1001..=1025)
        .map(|id| Employee {
            employee_id: id,
            age: rng.sample(Uniform::new(22, 60)),
            salary: rng.sample(Uniform::new(40000, 120000)),
            department: rng.sample(Slice::new(&departments).unwrap()).to_string(),
            years_of_experience: rng.sample(Uniform::new(1, 35)),
        })
        .collect();
    
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

// Function to measure performance
pub fn measure_performance(data: &[u32]) {
    let memory_before = get_memory_usage();
    let start_time = Instant::now();
    let result = process_data(data);
    let elapsed_time = start_time.elapsed();
    let memory_after = get_memory_usage();
    let memory_usage = memory_after - memory_before;

    println!("Processed Result: {}", result);
    println!("Running Time: {:.6?} seconds", elapsed_time);
    println!("Memory Usage During Execution: {:.6} MB", memory_usage);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dummy_df() {
        let (employees, mean_salary, median_salary, sum_salary, count_salary) = dummy_df();
        
        assert_eq!(employees.len(), 25); // Ensure 25 employees are generated
        assert!(mean_salary > 0.0); // Ensure mean salary is positive
        assert!(median_salary > 0.0); // Ensure median salary is positive
        assert!(sum_salary > 0); // Ensure sum of salaries is positive
        assert_eq!(count_salary, employees.len()); // Ensure count matches length of employee vector
    }

    #[test]
    fn test_process_data() {
        let data = [1, 2, 3, 4];
        let result = process_data(&data);
        assert_eq!(result, 10); // 1 + 2 + 3 + 4 = 10
    }

    #[test]
    fn test_get_memory_usage() {
        let memory_usage = get_memory_usage();
        assert!(memory_usage > 0.0, "Memory usage should be greater than 0");
    }
}