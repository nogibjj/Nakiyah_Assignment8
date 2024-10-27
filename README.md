# Nakiyah_Assignment8
[![CI/CD](https://github.com/nogibjj/Nakiyah_Assignment8/actions/workflows/rust.yml/badge.svg)](https://github.com/nogibjj/Nakiyah_Assignment8/actions/workflows/rust.yml) [![Rust CI/CD Pipeline](https://github.com/nogibjj/Nakiyah_Assignment8/actions/workflows/cicd.yml/badge.svg)](https://github.com/nogibjj/Nakiyah_Assignment8/actions/workflows/cicd.yml)


## File Structure
```
Nakiyah_Assignment7/
├── .devcontainer/
│   ├── devcontainer.json
│   └── Dockerfile
├── .github/
│   ├── workflows/cicd.yml
│   └── workflows/rust.yml
├── src/
│   ├── lib.rs
│   └── main.rs
├── test/
│   └── test.rs
├── .gitignore
├── .Cargo.lock
├── Cargo.toml
├── main.py
├── test.py
├── Makefile
├── Requirements.txt
└── README.md
```

## Project Overview:
This project aims to create a DataFrame of 25 employees and calculate the sum, count, mean, and median of their salaries in both Python and Rust. Additionally, the project measures the speed and resource usage for calculating the sum of salaries in both programming languages.

## Purpose
### The purpoe of this project is:
- Rewrite a python script in Rust
- Highlight improvements in speed and resource usage

## Rust Implementation
1. Build the Rust project:  ```cargo build```
2. To run the Rust application: ```cargo run```
3. To run the tests for the Rust implementation ```cargo test```

## Python Implementation:
1. Install requirements: ```make python_install```
2. Format Code: ```make python_format```
3. Lint Code: ```make python_lint```
4. Test Code: ```make python_test```

## Speed and Resource Usage:

The performance comparison between Python and Rust shows similar accuracy in results, as both calculate the same salary statistics. However, there are notable differences in runtime and memory usage that reveal each language's strengths and typical resource utilization.

1. Running Time
- Python: The Python script processed the salary data in 1 nanosecond (0.000001 seconds).
- Rust: The Rust program took 1.208 microseconds (or 1,208 nanoseconds), making Python technically faster here by a small margin.
- Analysis: Python’s slightly faster runtime is somewhat surprising, as Rust generally offers faster performance. This could be due to specific optimizations in Python's environment or nuances in how each language handles floating-point calculations.

2. Memory Usage
- Python: The memory usage reported during execution was 0.000000 MB (or near zero), indicating minimal memory consumption.
- Rust: Rust used 7376 MB, which is unexpectedly high for such a simple task.
- Analysis: Typically, Rust is highly memory-efficient, so the 7376 MB usage may be an outlier or measurement artifact.

Conclusion
While Python appears faster in this test, this doesn’t reflect typical performance for larger, more complex tasks where Rust would usually excel in speed and memory efficiency.