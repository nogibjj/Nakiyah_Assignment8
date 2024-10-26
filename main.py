import pandas as pd
import numpy as np
import psutil
import time

# Function to create the dummy DataFrame
def dummyDF():
    data_dict = {
        'Employee_ID': range(1001, 1026),
        'Age': np.random.randint(22, 60, size=25),
        'Salary': np.random.randint(40000, 120000, size=25),
        'Department': np.random.choice(['HR', 'Finance', 'IT', 'Marketing', 'Operations'], size=25),
        'Years_of_Experience': np.random.randint(1, 35, size=25)
    }
    df = pd.DataFrame(data_dict)
    
    # Calculate statistics for Salary column
    salary_column = df['Salary']
    mean_salary = salary_column.mean()
    median_salary = salary_column.median()
    sum_salary = salary_column.sum()
    count_salary = salary_column.count()

    return df, mean_salary, median_salary, sum_salary, count_salary

# Function to get memory usage (in MB)
def get_memory_usage():
    process = psutil.Process()
    mem_info = process.memory_info().rss
    return mem_info / (1024 * 1024)  # Convert bytes to MB

# Function to measure performance with memory and time precision
def measure_performance(data):
    memory_before = get_memory_usage()
    start_time = time.perf_counter()  # High-precision timer
    result = sum(data)
    elapsed_time = time.perf_counter() - start_time
    memory_after = get_memory_usage()
    memory_usage = memory_after - memory_before  # Focused memory usage during the task
    
    print(f"Processed Result (Sum of Data): {result}")
    print(f"Running Time: {elapsed_time:.6f} seconds")
    print(f"Memory Usage During Execution: {memory_usage:.6f} MB")

# Example usage
df, mean_salary, median_salary, sum_salary, count_salary = dummyDF()
print("DataFrame and statistics:", df, mean_salary, median_salary, sum_salary, count_salary, sep="\n")

# Sample data to test performance
sample_data = df['Age'].tolist()
measure_performance(sample_data)
