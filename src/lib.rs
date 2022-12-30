//! Library crate implmenting a backend for Conway's Game of Life (See
//! <https://en.wikipedia.org/wiki/Conway's_Game_of_Life>). The backend
//! is simple and provides a pre-defined width/height grid, i.e. it does not expand.
//! This crate does not implmement a frontend to actually display the grid,
//! you will have to use a separate crate for that or write your own.
//! # Usage example
//! ```
//! use conlife::{Grid, Object};
//!
//! let mut grid = Grid::new(16, 16);
//! let glider = Object::from_string("(0,2) (1,2) (2,2) (1,0) (2,1)");
//! // Load glider at (0,0)
//! grid.load_object(&glider, (0,0));
//! // advance by 10 generations
//! for _ in 0..10 {
//!     grid.advance();
//! }
//!
//! /* More code here to e.g. display the grid using a frontend */
//!
//!
//! ```
/// Struct representing a simple cell on a grid. When initialising
/// a [`Grid`], the neighbour indices for each cell are pre-calculated.
#[derive(Debug, Clone)]
pub struct Cell {
    neighbour_indices: Vec<(usize, usize)>,
    pub alive: bool,
}

impl Cell {
    fn new() -> Self {
        Self {
            alive: false,
            neighbour_indices: vec![],
        }
    }
    fn neighbour_count(&self, cells: &Vec<Vec<Cell>>) -> u32 {
        let mut neighbour_count = 0;
        for &position in &self.neighbour_indices {
            neighbour_count += cells[position.0][position.1].alive as u32;
        }

        neighbour_count
    }
}

/// The main struct provided by this crate. A grid contains many [`Cell`]s,
/// each of which can be alive or dead.
#[derive(Debug)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Vec<Cell>>,
}

impl Grid {
    /// Initialise a new grid. Use this instead of manually creating a new instance,
    /// as this function will pre-calculate the neighboiur indices for each cell.
    pub fn new(width: u32, height: u32) -> Self {
        let mut cells = vec![];
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                row.push(Cell::new())
            }
            cells.push(row);
        }
        let mut grid = Self {
            width,
            height,
            cells,
        };

        grid.compute_neighbour_indices();
        grid
    }

    /// Advance the grid by one generation.
    pub fn advance(&mut self) {
        let old_cells = self.cells.clone();
        for row in self.cells.iter_mut() {
            for cell in row {
                match cell.neighbour_count(&old_cells) {
                    0..=1 => {
                        cell.alive = false;
                    }
                    3 => {
                        cell.alive = true;
                    }
                    4.. => {
                        cell.alive = false;
                    }
                    _ => {}
                };
            }
        }
    }

    fn compute_neighbour_indices(&mut self) {
        for (x, row) in self.cells.iter_mut().enumerate() {
            for (y, cell) in row.iter_mut().enumerate() {
                let mut x_indices = vec![x];
                let mut y_indices = vec![y];

                if x != 0 {
                    x_indices.push(x - 1);
                }

                if x != self.width as usize - 1 {
                    x_indices.push(x + 1);
                }

                if y != 0 {
                    y_indices.push(y - 1);
                }

                if y != self.height as usize - 1 {
                    y_indices.push(y + 1);
                }

                for neighbour_x in x_indices {
                    for &neighbour_y in &y_indices {
                        if neighbour_x != x || neighbour_y != y {
                            cell.neighbour_indices.push((neighbour_x, neighbour_y));
                        }
                    }
                }
            }
        }
    }

    /// Load an [`Object`] into the grid a the specified position position
    pub fn load_object(&mut self, object: &Object, offset: (usize, usize)) {
        for (x, y) in &object.pixels {
            self.cells
                .get_mut(*y + offset.1)
                .unwrap()
                .get_mut(*x + offset.0)
                .unwrap()
                .alive = true;
        }
    }
}

/// Struct representing objects that can be loaded onto the grid.
/// You can for example load just one object, and then that object represents your entire
/// initial starting state for the grid, or you can for example have one object that represents
/// a glider, and load two gliders onto the grid at different positions.
pub struct Object {
    pub pixels: Vec<(usize, usize)>,
}

impl Object {
    /// Load an object from a file, usually with a `.life` extension, but this is not required.
    /// [`Self::from_string`] calls this function under the hood,
    /// so you can refer to its documentation to see what the format of the string should be.
    /// Sample files defining various objects can be found at <https://github.com/christofferaakre/conlife/tree/master/objects>.
    pub fn from_file(filename: &str) -> Self {
        let file_contents = std::fs::read_to_string(filename).expect("Failed to read file");
        Self::from_string(file_contents.as_str())
    }

    /// Load an object from a string. The string should contain ordered (x,y) coordinate pairs, separated by whitespace.
    /// Below is an example that defines a glider:
    /// `(0,2) (1,2) (2,2) (1,0) (2,1)`
    pub fn from_string(buffer: &str) -> Object {
        let mut pixels = vec![];
        let buffer = buffer.replace("(", "");
        let buffer = buffer.replace(")", "");
        let coords = buffer.split_whitespace();
        for coord in coords {
            let mut coord_vals = coord.split(",");
            let x: usize = coord_vals
                .next()
                .expect("Failed to get next value in comma split")
                .parse()
                .expect("Failed to parse coordinate to usize");
            let y: usize = coord_vals
                .next()
                .expect("Failed to get next value in comma split")
                .parse()
                .expect("Failed to parse coordinate to usize");
            pixels.push((x, y));
        }
        Self { pixels }
    }
}
