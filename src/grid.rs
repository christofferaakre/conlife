//! Module exposing the API for creating and interacting with a grid of cells.
//!
use crate::Object;
/// Struct representing a simple cell on a grid. When initialising
/// a [`Grid`], the neighbour indices for each cell are pre-calculated.
#[derive(Debug, Clone, PartialEq, Eq)]
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
            neighbour_count += cells[position.1][position.0].alive as u32;
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

    /// Prints the coordiantes of the currently alive cells,
    /// for debugging purposes
    pub fn print_alive_cells(&self) {
        println!("------- Alive cells ---------");
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell.alive {
                    print!("({x}, {y}), ");
                }
            }
        }
        println!("-----------------------------");
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
                    2 => {}
                    3 => {
                        cell.alive = true;
                    }
                    4.. => {
                        cell.alive = false;
                    }
                };
            }
        }
    }

    fn compute_neighbour_indices(&mut self) {
        for (y, row) in self.cells.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                let mut x_indices = vec![x];
                let mut y_indices = vec![y];

                if x != 0 {
                    let i = x - 1;
                    if self.width > i as u32 {
                        x_indices.push(i);
                    }
                }

                if x != self.width as usize - 1 {
                    let i = x + 1;
                    if self.width > i as u32 {
                        x_indices.push(i);
                    }
                }

                if y != 0 {
                    let i = y - 1;
                    if self.height > i as u32 {
                        y_indices.push(i);
                    }
                }

                if y != self.height as usize - 1 {
                    let i = y + 1;
                    if self.height > i as u32 {
                        y_indices.push(i);
                    }
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
        for (x, y) in &object.coordinates {
            let error_msg = format!(
                "Position {:?} is out of bounds for grid of size ({}, {})",
                (*x + offset.0, *y + offset.1),
                self.width,
                self.height
            );
            self.cells
                .get_mut(*y + offset.1)
                .expect(&error_msg)
                .get_mut(*x + offset.0)
                .expect(&error_msg)
                .alive = true;
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::Object;

    #[test]
    fn empty_grid_advance() {
        let mut grid = Grid::new(16, 16);
        let initial_cells = grid.cells.clone();
        grid.advance();
        assert_eq!(initial_cells, grid.cells);
    }

    #[test]
    fn full_grid_advance() {
        let mut grid = Grid::new(8, 8);
        // set all cells to be alive
        for row in grid.cells.iter_mut() {
            for cell in row {
                cell.alive = true;
            }
        }
        let initial_cells = grid.cells.clone();
        grid.advance();
        // only corner cells should survice
        for (y, row) in grid.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if [0, grid.width - 1].contains(&(x as u32))
                    && [0, grid.height - 1].contains(&(y as u32))
                {
                    assert!(cell.alive);
                } else {
                    assert!(!cell.alive);
                }
            }
        }
    }

    #[test]
    fn load_glider() {
        let mut grid = Grid::new(8, 8);
        let glider = Object::from_file("objects/glider.life").expect("Failed to load glider");
        grid.load_object(&glider, (0, 0));

        let alive = vec![(0, 2), (1, 2), (2, 2), (1, 0), (2, 1)];

        for (y, row) in grid.cells.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                let coord = (x, y);
                assert_eq!(cell.alive, alive.contains(&coord));
            }
        }
    }

    #[test]
    fn load_glider_at_offset() {
        let mut grid = Grid::new(64, 64);
        let glider = Object::from_file("objects/glider.life").expect("Failed to load glider");
        let offset = (1, 1);
        grid.load_object(&glider, offset);

        let alive: Vec<(usize, usize)> = glider
            .coordinates
            .iter()
            .map(|(x, y)| (x + offset.0, y + offset.1))
            .collect();

        grid.print_alive_cells();

        for (y, row) in grid.cells.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                let coord = (x, y);
                assert_eq!(cell.alive, alive.contains(&coord));
            }
        }
    }

    #[test]
    fn glider_advance() {
        let mut grid = Grid::new(8, 8);
        let glider = Object::from_file("objects/glider.life").expect("Failed to load glider");
        grid.load_object(&glider, (0, 0));
        grid.advance();

        let alive = vec![(0, 1), (1, 2), (1, 3), (2, 1), (2, 2)];

        for (y, row) in grid.cells.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                let coord = (x, y);
                assert_eq!(cell.alive, alive.contains(&coord));
            }
        }
    }
}
