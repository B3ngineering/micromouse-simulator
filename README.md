# Micromouse Simulater

This project is a maze-solving simulator for the micromouse maze-solving competition. It's built using Rust, Cargo, and Druid. For more information on the competition, see [here](https://en.wikipedia.org/wiki/Micromouse). 

The purpose of this simulator is to allow micromouse enthusiasts to develop pathfinding algorithms in a simulated space, and compare their real world performance to other algorithms.
I wanted to learn more about path finding algorithms as well as different heuristics for them, and this project allowed me to do so while building my skills in Rust and user interface development.

## Core Features

The current implementation of this simulator supports navigation of a variety of mazes, and comes with the A-Star algorithm and Flood Fill algorithm already implemented.
These can be called using the buttons on the user interface, and if more algorithms are implemented they will be runnable the same way. Simple navigation of the mouse can be done using the
North, South, East, and West buttons. There is a Reset button to return to the original state, and a Traverse button to have the mouse follow the current path and generate a score. 

## Demo

[Here](https://youtu.be/1F_-CBph6FM) is a Youtube link displaying the current functionality.

## Getting Started

- Rust and Cargo can be installed according to [these](https://doc.rust-lang.org/cargo/getting-started/installation.html) instructions.
- Druid can be installed from [here](https://github.com/linebender/druid).
- After cloning this repository, the application will be runnable with "cargo run" from the home directory.

