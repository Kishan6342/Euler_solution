# 🧮 Euler Method Solver for ODE in Rust

## 📘 Problem Statement

Solve the following initial value problem (IVP) using the **Euler method**:

\[
y' = \cos(t) - y,\quad 0 \leq t \leq 5,\quad y(0) = 1
\]

### 📌 Analytical Solution

This differential equation is linear and solvable analytically using integrating factors.

The exact solution is:

\[
y(t) = \frac{1}{2}(\cos(t) + \sin(t)) + \frac{1}{2}e^{-t}
\]

This analytical solution is used to compare the accuracy of the numerical Euler method.

---

## 🗂️ Repository Structure

- `src/`  
  Contains the Rust source file which implements the **Euler method** to solve the given ODE.

- `main.rs`  
  A separate Rust program used to **generate CSV files** for visualization. It computes numerical and analytical values and stores them alongside their difference.

- `output_20.csv`  
- `output_100.csv`  
- `output_1000.csv`  
  These CSV files contain results for 20, 100, and 1000 time steps respectively.

- `visualization.ipynb`  
  A Python Jupyter notebook that loads the CSV files and **visualizes** the results. It includes:
  - Comparison of analytical and numerical solutions
  - Plot of error vs time for different step sizes

---

## 📊 CSV File Format

Each CSV contains **4 columns**:

| Column              | Description                                     |
|---------------------|-------------------------------------------------|
| `t`                 | Time value \( t_i \)                            |
| `analytical`        | Analytical solution \( y(t_i) \)               |
| `numerical`         | Euler method result \( y_i \)                  |
| `error`             | Absolute difference: \( |y(t_i) - y_i| \)       |

These files are used to generate plots to analyze accuracy and error behavior of the Euler method.

---

## ▶️ How to Run

### 🦀 Using Rust

To build and run the solver:

```bash
cargo run --release
