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

### Configuration
#### Speed
You may also adjust the speed of the simulation (or more accurately, the interval between
updates of the simulation) through the slider on the bottom left. The number indicates the
approximate number of milliseconds between each update of the grid.

#### Simulation Type
The simulation by default is in the "VOID" type. This means that cells, once they reach the
boundary of the grid, will "disappear" into a void. We have also implemented a "WRAP" type,
which wraps the cells back around into the grid. You can toggle between these types using 
the button that displays either "VOID" or "WRAP" on the lower right corner.

### Movement
You can zoom in and out of the grid, showing more cells or less cells as you like, by use of
the mouse wheel or trackpad scroll.


