# Advent of Code - 2023 Rust

This Rust project is designed for solving [Advent of Code](https://adventofcode.com/) challenges.
It includes a script for initializing daily challenge directories,
fetching puzzle inputs, and a utility library where common operations can be placed.

## Getting Started

These instructions will get you a copy of the project up and running on your local
machine for development and testing purposes.

### Prerequisites

* Rust programming language environment. [Install Rust](https://www.rust-lang.org/tools/install).
* `curl` command-line tool for fetching puzzle inputs.

### Setup

1. Clone the repository:
    ```shell
    git clone https://github.com/nizos/aoc23
    cd aoc23
    ```
2. Environment Setup:
   
    Rename the `.env.example` file in the root directory of the project to `.env` and add your session cookie to it:

    ```shell
    AOC_SESSION="<your session cookie here>"
    ```
    To obtain your session cookie:
    * Log in to the Advent of Code website.
    * Open the Developer Tools (usually F12) in your browser.
    * Go to the Application/Storage tab and find the session cookie.

3. Starting a new day:

    Run the `start-day.sh` script to set up a new day's challenge:

    ```shell
    ./start-day.sh <day-number>
    ```

    For example, for day 1:

    ```shell
    ./start-day.sh 1
    ```

   This will create a new directory for the day, copy the template code, and fetch the day's input.

## Running the Solutions

To run a solution for a specific day:

```shell
cargo run --bin day_XX
```

Replace `XX` with the day number, for example, `cargo run --bin day_01` for day 1.

## Testing the Solutions

To test a specific day's solution:

```shell
cargo test --bin day_XX
```

## Formatting and Linting

To ensure your code follows Rust's style guidelines, use:

* **Formatting**: Run `cargo fmt` to format your code.
* **Linting**: Run `cargo clippy` for linting and catching common mistakes.

## Acknowledgments

Special thanks to [Raniz85](https://github.com/Raniz85) for creating the project starter functionality
used in this project.