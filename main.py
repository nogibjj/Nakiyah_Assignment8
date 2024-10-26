import pandas as pd
import numpy as np
import psutil
import time

# Function to create the dummy DataFrame
def dummyDF():
    dataDict = {
        "Employee_ID": range(1001, 1026),
        "Age": np.random.randint(22, 60, size=25),
        "Salary": np.random.randint(40000, 120000, size=25),
        "Department": np.random.choice(
            ["HR", "Finance", "IT", "Marketing", "Operations"], size=25
        ),
        "Years_of_Experience": np.random.randint(1, 35, size=25),
    }
    df = pd.DataFrame(dataDict)

    # Calculate statistics for Salary column
    salaryColumn = df["Salary"]
    salaryMean = salaryColumn.mean()
    salaryMedian = salaryColumn.median()
    salarySum = salaryColumn.sum()
    salaryCount = salaryColumn.count()

    return df, salaryMean, salaryMedian, salarySum, salaryCount


# Function to get memory usage (in MB)
def memoryUsage():
    process = psutil.Process()
    memoryInfo = process.memory_info().rss
    return memoryInfo / (1024 * 1024)  # Convert bytes to MB


# Function to measure performance with memory and time precision
def measurePerformance(data):
    memory_before = memoryUsage()
    start_time = time.perf_counter()  # High-precision timer
    result = sum(data)
    elapsed_time = time.perf_counter() - start_time
    memory_after = memoryUsage()
    memory_usage = memory_after - memory_before  # Focused memory usage during the task

    print(f"Processed Result (Sum of Data): {result}")
    print(f"Running Time: {elapsed_time:.6f} seconds")
    print(f"Memory Usage During Execution: {memory_usage:.6f} MB")


# Example usage
df_dummy, mean_salary, median_salary, sum_salary, count_salary = dummyDF()
print(
    "DataFrame and statistics:",
    df_dummy,
    mean_salary,
    median_salary,
    sum_salary,
    count_salary,
    sep="\n",
)

# Sample data to test performance
sampleData = df_dummy["Age"].tolist()
measurePerformance(sampleData)
