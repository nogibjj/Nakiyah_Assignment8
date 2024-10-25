import pandas as pd
import numpy as np

def dummyDF():
    # Creating a better quality DataFrame with meaningful data
    data = {
        'Employee_ID': range(1001, 1026),  # Employee IDs from 1001 to 1025
        'Age': np.random.randint(22, 60, size=25),  # Random ages between 22 and 60
        'Salary': np.random.randint(40000, 120000, size=25),  # Random salary between 40k and 120k
        'Department': np.random.choice(['HR', 'Finance', 'IT', 'Marketing', 'Operations'], size=25),  # Random department
        'Years_of_Experience': np.random.randint(1, 35, size=25)  # Random years of experience between 1 and 35
    }

    # Creating the DataFrame
    df = pd.DataFrame(data)

    # Select a column, e.g., 'Salary', and calculate its statistics
    salary_column = df['Salary']
    mean_salary = salary_column.mean()
    median_salary = salary_column.median()
    sum_salary = salary_column.sum()
    count_salary = salary_column.count()

    # Returning the DataFrame and statistics
    return df, mean_salary, median_salary, sum_salary, count_salary

# Example usage
data, meanSalary, medianSalary, sumSalary, countSalary = dummyDF()

