# PID Loop Demo in Rust

This Rust program demonstrates the implementation of Proportional-Integral-Derivative (PID) control loops in a simple console-based environment. The program simulates a scenario where you control a player character, and three PID-controlled entities follow the player’s movements.

## Features

- **Real-Time PID Control**: Each of the three follower characters uses a PID loop to adjust their position relative to the player.
- **Text-Based Display**: The positions of the player and the followers are displayed in the console, providing a clear visual representation of the PID control in action.
- **Interactive**: Move the player around, and observe how the PID-controlled characters respond, trying to follow the player as closely as possible.

## Purpose

This demo is designed for those interested in learning about PID control loops in Rust, providing a hands-on example of how these loops can be implemented and visualized.

## Getting Started

1. Clone the repository.
2. Build and run the project using Cargo.
3. Use the controls to move the player character and watch the PID-controlled followers in the console.

## Requirements

- Rust (latest stable version)
- Cargo (Rust's package manager)

## How It Works

The program initializes a player and three follower characters in a simple 2D space. As the player moves, the followers adjust their positions based on PID loops, which aim to minimize the error between their current position and the player’s position. This simulation helps in understanding how PID control can be used to achieve smooth and responsive movement.

## Contributions

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License.

