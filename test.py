import subprocess


def test_dummyDF():
    """Tests the dummyDF function."""
    result = subprocess.run(
        ["python3", "main.py"],  # Adjust the command as needed
        capture_output=True,
        text=True,
        check=True,
    )

    # Check that the command executed successfully
    assert result.returncode == 0, "Subprocess failed to execute the script."

    # Check if the output contains expected keywords
    assert "Performance results have been written to Python_Performance.md" in result.stdout


if __name__ == "__main__":
    test_dummyDF()
