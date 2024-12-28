# Rust Raw Pointer Vector Modification Bug

This repository demonstrates a common, yet subtle, error in Rust involving the misuse of raw pointers and vectors.  Modifying a vector's contents via a raw pointer after the vector has been reallocated leads to undefined behavior. This example showcases the error and its correction.

## Bug Description
The `bug.rs` file contains code that uses `as_mut_ptr()` to obtain a raw pointer to a vector's data.  It then uses this pointer to modify the vector's contents. This is unsafe because the vector's internal memory layout could change (e.g., if the vector needs to grow). Accessing the memory through the outdated pointer after reallocation is undefined behavior.

## Solution
The `bugSolution.rs` file demonstrates the correct way to modify a vector, which involves using safe Rust methods.  No raw pointers are involved in the solution.

## How to run
To run the examples, clone this repository and run `rustc bug.rs && ./bug` and `rustc bugSolution.rs && ./bugSolution`.