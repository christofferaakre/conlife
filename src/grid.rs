//! Module exposing the API for creating and interacting with a grid of cells.
use crate::Object;
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
        for (x, y) in &object.coordinates {
            self.cells
                .get_mut(*y + offset.1)
                .unwrap()
                .get_mut(*x + offset.0)
                .unwrap()
                .alive = true;
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::Object;

    #[test]
    fn load_glider() {
        let grid = Grid::new(16, 16);
        let glider = Object::from_file("objects/glider.life");
    }
}
