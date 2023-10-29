# leet-code-rs

This repository serves as an archive of my solutions to LeetCode problems, implemented in the Rust programming language. Each solution is stored in a separate branch named problem_N, where N represents the problem number.

## Folder Structure

The repository follows the following folder structure:

├ src/\
│ └── problem_N.rs\
└ tests/\
&emsp;└── problem_N.rs

- src/: This directory contains the implementation of the solution for each problem. Each problem has its own Rust file named problem_N.rs, where N represents the problem number.
- tests/: This directory holds the unit tests for each problem. Similar to the implementation, each problem also has its corresponding test file named problem_N.rs.

## Get Started

To explore the solutions or run the tests locally, you can follow these steps:

1.Clone the repository:

```shell
git clone https://github.com/rusha-mathing/leet-code-rs.git
```

2.Change into the cloned directory:

```shell
cd leet-code-rs
```

3.Check out the branch for a specific problem. For example, to view the solution for problem #42, run the following:

```shell
git checkout problem_42
```

4.Explore the code in the src/ directory for the implementation and the tests/ directory for the corresponding unit tests.

5.Optionally, you can run the tests for a specific problem using the Rust testing framework. For example, to run the tests for problem #42, run the command:

```shell
cargo test --test problem_42
```

Feel free to browse through the branches to find the solutions for the problems of your interest.
