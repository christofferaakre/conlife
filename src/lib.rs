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
//! let glider = Object::from_string("(0,2) (1,2) (2,2) (1,0) (2,1)").unwrap();
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

pub mod grid;
pub mod object;
pub use grid::*;
pub use object::*;
