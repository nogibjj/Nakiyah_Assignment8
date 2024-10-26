// Import the library module
use main_rust::{dummy_df, measure_performance};

fn main() {
    let (employees, mean_salary, median_salary, sum_salary, count_salary) = dummy_df();
    
    println!("DataFrame and statistics:");
    for employee in &employees {
        println!("ID: {}, Age: {}, Salary: {}, Dept: {}, Experience: {}",
            employee.employee_id, employee.age, employee.salary, employee.department, employee.years_of_experience);
    }
    println!("Mean Salary: {:.2}", mean_salary);
    println!("Median Salary: {:.2}", median_salary);
    println!("Sum of Salaries: {}", sum_salary);
    println!("Count of Salaries: {}", count_salary);

    // Sample data to test performance
    let sample_data: Vec<u32> = employees.iter().map(|e| e.age).collect();
    
    // Measure performance of summing ages
    println!("About to call measure_performance...");
    measure_performance(&sample_data);
    println!("Finished measure_performance call.");
}
