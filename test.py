import unittest
import subprocess


class TestEmployeeFunctions(unittest.TestCase):
    def test_dummyDF_output(self):
        """Test if the dummyDF function runs without errors."""
        result = subprocess.run(["python3", "main.py"], capture_output=True, text=True)

        # Check that the command executed successfully
        self.assertEqual(result.returncode, 0)

        # Check if the output contains expected keywords (adjust as needed)
        self.assertIn("DataFrame and statistics:", result.stdout)
        self.assertIn("Processed Result (Sum of Data):", result.stdout)

    def test_memoryUsage_output(self):
        """Test if the memory usage is reported correctly."""
        result = subprocess.run(["python3", "main.py"], capture_output=True, text=True)

        # Check that the command executed successfully
        self.assertEqual(result.returncode, 0)

        # Check if the output mentions memory usage
        self.assertIn("Memory Usage During Execution:", result.stdout)

    def test_measurePerformance_output(self):
        """Test if performance measurement runs without errors."""
        result = subprocess.run(["python3", "main.py"], capture_output=True, text=True)

        # Check that the command executed successfully
        self.assertEqual(result.returncode, 0)

        # Check if the output contains performance-related information
        self.assertIn("Running Time:", result.stdout)


if __name__ == "__main__":
    unittest.main()
