import pandas as pd
import psutil
import time

# Function to create the dummy DataFrame
def dummyDF():
    df = pd.DataFrame({
        "Employee_ID": range(1001, 1026),
        "Age": [25, 30, 35, 40, 28, 45, 32, 29, 31, 34, 27, 26, 33, 38, 36, 37, 39, 42, 41, 43, 44, 46, 48, 49, 50],
        "Salary": [50000, 60000, 70000, 80000, 55000, 90000, 65000, 62000, 58000, 76000, 
                   53000, 54000, 63000, 85000, 72000, 71000, 78000, 83000, 82000, 87000, 
                   88000, 91000, 95000, 98000, 100000],
        "Department": ["HR", "Finance", "IT", "Marketing", "Operations", "Finance", "IT", "HR", "Marketing", 
                                  "Operations", "Finance", "IT", "HR", "Marketing", "Operations", "Finance", "IT", "HR", 
                                  "Marketing", "Operations", "Finance", "IT", "HR", "Marketing", "Operations"],
        "Years_of_Experience": [5, 10, 15, 20, 7, 25, 12, 8, 9, 14, 6, 4, 11, 18, 16, 17, 19, 22, 21, 23, 24, 26, 28, 29, 30]
    })

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
    memoryBefore = memoryUsage()
    startTime = time.perf_counter()  # High-precision timer
    result = sum(data)
    elapsedTime = time.perf_counter() - startTime
    memoryAfter = memoryUsage()
    memoryUse = memoryAfter - memoryBefore  # Focused memory usage during the task

    return result, elapsedTime, memoryUse

# Create dummy DataFrame and calculate statistics
dfDummy, meanSalary, medianSalary, sumSalary, countSalary = dummyDF()

# Print the salary statistics
print("Salary Statistics:",
      f"Mean: {meanSalary}", f"Median: {medianSalary}",
      f"Sum: {sumSalary}", f"Count: {countSalary}", sep="\n")

# Sample data to test performance
sampleData = dfDummy["Salary"].tolist()
totalSalary, runningTime, memoryUsageDuringExecution = measurePerformance(sampleData)

# Print performance measurement
print("\n")
print(f"Processed Result (Total Salary): {totalSalary}")
print(f"Running Time: {runningTime:.6f} seconds")
print(f"Memory Usage During Execution: {memoryUsageDuringExecution:.6f} MB")

# Create a Markdown file for Python performance analysis
performance_analysis = f"""
# Python Performance Analysis
## Salary Statistics

- Mean Salary: {meanSalary:.2f}
- Median Salary: {medianSalary:.2f}
- Sum of Salaries: {sumSalary}
- Count of Salaries: {countSalary}

## Performance Measurement

- Processed Result (Total Salary): {totalSalary}
- Running Time: {runningTime:.6f} seconds
- Memory Usage During Execution: {memoryUsageDuringExecution:.6f} MB
"""

# Write the performance analysis to a Markdown file
with open("Python_Performance.md", "w") as file:
    file.write(performance_analysis)

print("Performance results have been written to Python_Performance.md")
