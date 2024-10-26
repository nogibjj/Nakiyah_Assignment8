// Import the library module
use main_rust::{dummy_df};
use std::fs::File;
use std::io::Write; // For write_all method
use std::time::Instant;

fn write_to_markdown(content: &str, filename: &str) {
    let mut file = File::create(filename).expect("Unable to create file");
    file.write_all(content.as_bytes()).expect("Unable to write data");
}

fn main() {
    let (employees, mean_salary, median_salary, sum_salary, count_salary) = dummy_df();

    // Sample data to test performance
    let sample_data: Vec<u32> = employees.iter().map(|e| e.salary).collect();

    // Measure performance (total salary) and capture output
    let memory_before = main_rust::get_memory_usage();
    let start_time = Instant::now();
    let result = main_rust::process_data(&sample_data);
    let elapsed_time = start_time.elapsed();
    let memory_after = main_rust::get_memory_usage();
    let memory_usage = memory_after - memory_before;

    // Prepare performance measurement data
    let input_data = format!(
"# Rust Performance Analysis\n\
## Salary Statistics\n\n\
        - Mean Salary: {:.2}\n\
        - Median Salary: {:.2}\n\
        - Sum of Salaries: {}\n\
        - Count of Salaries: {}\n\n\
        
## Performance Measurement\n\n\
         - Processed Result (Total Salary): {}\n\
         - Running Time**: {:.6?} seconds\n\
         - Memory Usage During Execution: {:.6} MB\n",
         mean_salary, median_salary, sum_salary, count_salary, result, elapsed_time, memory_usage
    );

    // input data for output
    let output_content = format!("{}", input_data);
    
    // Write to markdown file
    write_to_markdown(&output_content, "Rust_Perfomance.md");
    
    println!("Performance results have been written to Rust_Performance.md");
}
