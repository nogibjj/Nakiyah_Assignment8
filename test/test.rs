use main_rust::{dummy_df, process_data, get_memory_usage};

#[cfg(test)]
mod tests {
    use super::*;
//    use rand::Rng;
    
    // Tests the dummy_df function to ensure it generates 25 employees and that calculated mean, median, sum, and count of salaries are all greater than zero.
    #[test]
    fn test_dummy_df() {
        let (employees, mean_salary, median_salary, sum_salary, count_salary) = dummy_df();
        
        // Ensure we have the expected number of employees
        assert_eq!(employees.len(), 25);

        // Check that mean, median, sum, and count are calculated correctly
        assert!(mean_salary > 0.0);
        assert!(median_salary > 0.0);
        assert!(sum_salary > 0);
        assert_eq!(count_salary, employees.len());
    }

    // Tests the process_data function with basic input to check if the sum is calculated correctly.
    #[test]
    fn test_process_data_basic() {
        let data = [1, 2, 3, 4];
        let result = process_data(&data);
        assert_eq!(result, 10); // 1 + 2 + 3 + 4 = 10
    }

    // Similar to the basic test, but with larger numbers to verify that it can handle them correctly.
    #[test]
    fn test_process_data_large_values() {
        let data = [100, 200, 300, 400];
        let result = process_data(&data);
        assert_eq!(result, 1000); // 100 + 200 + 300 + 400 = 1000
    }

    // Tests the process_data function with an empty vector to confirm that it returns zero.
    #[test]
    fn test_process_data_empty() {
        let data: Vec<u32> = vec![];
        let result = process_data(&data);
        assert_eq!(result, 0); // Sum of an empty array should be 0
    }

    // Test for get_memory_usage function - Verifies that the memory usage function returns a value greater than zero.
    #[test]
    fn test_get_memory_usage() {
        let memory_usage = get_memory_usage();
        assert!(memory_usage > 0.0, "Memory usage should be greater than 0");
    }
}
