# conway
Conway's Game of Life in Rust with SDL2.

## Installation
### `cargo`
After cloning this repository, use `cargo build` to build a binary or `cargo run` to run 
the application. Note that this application requires the SDL2 native library to 
build.

### Binaries
We offer pre-built binaries for x86-64 Linux [here](https://github.com/Injng/conway/releases/latest).

## Usage
Draw the pattern on the grid by clicking in the cells. Each click flips the state of the cell; 
i.e. a nonactive cell, when clicked, becomes active, and vice versa.

To begin the simulation, click the play button at the bottom of the grid. To pause it, click 
the pause button that will be located once the simulation has started playing. Note also that
the grid size on screen may change as you resize the window, but the back-end simulation runs
on a fixed 60x60 grid that wraps around on borders.
