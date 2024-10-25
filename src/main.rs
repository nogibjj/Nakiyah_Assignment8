// src/main.rs

use main_rust::{dummy_df, measure_performance}; // Use the actual crate name defined in Cargo.toml

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate the dummy DataFrame and its statistics
    let (df, mean_salary, median_salary, sum_salary, count_salary) = dummy_df()?;
    
    // Display DataFrame and salary statistics
    println!("DataFrame:\n{}", df);
    println!("\nSalary Statistics:");
    println!("Mean Salary: {}", mean_salary);
    println!("Median Salary: {}", median_salary);
    println!("Sum of Salaries: {}", sum_salary);
    println!("Count of Salaries: {}", count_salary);

    // Process the salary data for performance measurement
    // Get the Salary column as i32
    let salary_series = df.column("Salary")?.i32()?;

    // Convert the ChunkedArray to a Vec<i32>
    let salary_vec: Vec<i32> = salary_series.into_iter().filter_map(|opt| opt).collect();

    // Measure performance on the Salary data
    measure_performance(&salary_vec);

    Ok(())
}

