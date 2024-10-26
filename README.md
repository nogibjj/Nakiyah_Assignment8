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

