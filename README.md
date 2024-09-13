# Intro to Rust: Oxidize the Classic Command-Line App

Starter code (and solution) for Intro to Rust workshop at Hack the North.
The main branch contains the code to follow along with the workshop and the
solution branch contains the final working code for the password manager.

## Installing Rust

To install Rust:
- on Windows, go to [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
and install Rustup
- on Mac/Linux/WSL, run the following in your terminal
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
which should install Rustup to your machine

## How to Run/Build

To run the project, run `cargo run -- --help` in your terminal when in the project directory,
which should show the help menu.

To run the project with any other arguments, run `cargo run -- [-a | --argName] argVal`,
with the arguments after the `--`.
