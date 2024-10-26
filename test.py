from main import dummyDF  # Adjust the import based on your file structure

def test_dummyDF():
    """Test if the dummyDF function runs without errors and returns expected output."""
    df, mean_salary, median_salary, sum_salary, count_salary = dummyDF()

    # Check if the DataFrame has the correct shape
    assert df.shape == (25, 5), "DataFrame should have 25 rows and 5 columns."

    # Check if the statistics are calculated correctly
    assert mean_salary == df['Salary'].mean(), "Mean salary should match."
    assert median_salary == df['Salary'].median(), "Median salary should match."
    assert sum_salary == df['Salary'].sum(), "Salary sum should match."
    assert count_salary == df['Salary'].count(), "Salary count should match."

    print("All tests passed!")

if __name__ == "__main__":
    test_dummyDF()  # Run the test when the script is executed
