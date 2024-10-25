use polars::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;
use sysinfo::{System, SystemExt};

pub fn dummy_df() -> Result<(DataFrame, f64, f64, i64, usize), PolarsError> {
    // Employee ID range
    let employee_id: Vec<i32> = (1001..1026).collect();

    // Random ages between 22 and 60
    let age_dist = Uniform::new_inclusive(22, 60);
    let mut rng = thread_rng();
    let age: Vec<i32> = (0..25).map(|_| age_dist.sample(&mut rng)).collect();

    // Random salary between 40k and 120k
    let salary_dist = Uniform::new_inclusive(40000, 120000);
    let salary: Vec<i32> = (0..25).map(|_| salary_dist.sample(&mut rng)).collect();

    // Random department choices
    let departments = vec!["HR", "Finance", "IT", "Marketing", "Operations"];
    let department: Vec<&str> = (0..25)
        .map(|_| *departments.choose(&mut rng).unwrap())
        .collect();

    // Random years of experience between 1 and 35
    let exp_dist = Uniform::new_inclusive(1, 35);
    let years_of_experience: Vec<i32> = (0..25).map(|_| exp_dist.sample(&mut rng)).collect();

    // Creating the DataFrame
    let df = DataFrame::new(vec![
        Series::new("Employee_ID", employee_id),
        Series::new("Age", age),
        Series::new("Salary", salary.clone()),
        Series::new("Department", department),
        Series::new("Years_of_Experience", years_of_experience),
    ])?;

    // Calculating statistics for the Salary column
    let mean_salary = salary.iter().map(|&x| x as f64).sum::<f64>() / salary.len() as f64;
    let median_salary = {
        let mut sorted_salary = salary.clone();
        sorted_salary.sort_unstable();
        sorted_salary[sorted_salary.len() / 2] as f64
    };
    let sum_salary: i64 = salary.iter().map(|&x| x as i64).sum();
    let count_salary = salary.len();

    Ok((df, mean_salary, median_salary, sum_salary, count_salary))
}

pub fn process_data(data: &[i32]) -> i64 {
    data.iter()
        .map(|&x| (x as i64) * (x as i64)) // Cast to i64 before squaring
        .filter(|&x| x <= 1000) // You can keep this as it is
        .sum() // Sum will be an i64
}

pub fn get_memory_usage() -> f64 {
    let mut sys = System::new_all();
    sys.refresh_memory();

    let used_memory = sys.used_memory();
    used_memory as f64 / (1024.0 * 1024.0) // Convert to MB
}

pub fn measure_performance(data: &[i32]) {
    let start_time = Instant::now();

    let result = process_data(data);

    let elapsed_time = start_time.elapsed();
    let memory_usage = get_memory_usage();

    // Print the results
    println!("Processed Result: {}", result);
    println!("Running Time: {:.6?} seconds", elapsed_time);
    println!("Memory Usage: {:.6} MB", memory_usage);
}
