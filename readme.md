# Tic_tac_toe_online

Multiplayer tic-tac-toe game written in Rust with only the standard library (no external dependencies).

## Table of Contents
- Overview
- Features
- Installation
- User Guide
- Code Guide and Extra Notes

## Overview

This Rust app allows you to play Tic Tac Toe online. However, this code only allows users to play with other running tic-tac-toe games on the same computer. To play it within your local Wi-Fi network, you should adjust the `src/online.rs` file.

## Features

- Tic-tac-toe game
- TCP connection for online gameplay
- Basic error handling

## Installation

First, download the GitHub repository. After that, follow the steps listed below:



```bash
cd path/to_code
cargo build --release
executable_name
```
or 
```bash
cd path/to_code
cargo run 
```

## User Guide
1. [install](#installation)
2. if you want to be X type 1 or for O type 2
# Super important Note:
For the game to run properly Player_O(2) should start the game and see the  waiting for response message or other wise Player_X's game will crash with an error message stating that there is no server running!!!!!
3.The game uses the basic tic-tac-toe rules. For the first input, enter the row number (0-2) and for the second input, enter the column number (0-2).

## Code Guide and Extra Notes:
This project only leverages the standard library that comes with Rust.
- std::net for tcp connections in [online.rs]{}
- std::io for user input and printing the game and more
- std::thread for sleep function not for multiple connections


While design the game logic and more I have excessively used if and else statements, and for some parts of the code this might be inefficient so if you have any suggestions I am open to them.

The main goal with this project was to learn rust(my first rust project) and simple network programming like tcp, socket, and HTTP.
