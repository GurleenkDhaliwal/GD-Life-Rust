# Life

## Submitted by: Gurleen Dhaliwal

## Overview
This project implements Conway's Game of Life on an MB2 board using Rust. The game starts with a randomly generated board, and players can re-randomize or complement the board using buttons on the MB2device.

## Implementation Details
- Board Initialization : The game board initializes with a random state.
- Button Interactions : Button A re-randomizes the board, and Button B complements it.
- Display Updates : The current state of the game board is displayed using the MB2 board's LEDs.

## Observations
While finishing "Life", I realized  Rust's prowess in embedded systems. It was initially confusing what to do with the chip and how to prepate the environment on my terminal for it to work. But while following the step by step document I was able to figure it out. Overall, it is a great learning experience.

## How to Run
1. Ensure Rust and Cargo are installed.
2. Clone the repository and navigate into the project directory.
3. Build the project using `cargo build --target thumbv7em-none-eabihf`.
4. Flash the binary to the MB2 board with `cargo embed --target thumbv7em-none-eabihf`.


