use image::{Rgb, RgbImage};

#[derive(Debug, Clone)]
pub struct Cell {
    neighbour_indices: Vec<(usize, usize)>,
    alive: bool,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            alive: false,
            neighbour_indices: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Vec<Cell>>,
}

const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
const RED: Rgb<u8> = Rgb([255, 0, 0]);

pub enum HorizontalEdge {
    Left,
    Right,
    NoEdge,
}

pub enum VerticalEdge {
    Top,
    Bottom,
    NoEdge,
}

impl Grid {
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

    pub fn compute_neighbour_indices(&mut self) {
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

    pub fn draw(&self) -> RgbImage {
        let mut image = RgbImage::new(self.width, self.height);
        for (x, row) in self.cells.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                let color = match cell.alive {
                    true => GREEN,
                    false => RED,
                };

                image.put_pixel(x as u32, y as u32, color);
            }
        }

        image
    }
}

const WIDTH: u32 = 16;
const HEIGHT: u32 = 16;

const TARGET_WIDTH: u32 = 512;
const TARGET_HEIGHT: u32 = 512;

fn main() {
    let mut grid = Grid::new(WIDTH, HEIGHT);
    println!("{:?}", grid);

    grid.cells[0][0].alive = true;
    let image = grid.draw();

    let filename = "out/out.png";

    let image = image::imageops::resize(
        &image,
        TARGET_WIDTH,
        TARGET_HEIGHT,
        image::imageops::FilterType::Nearest,
    );

    image.save(filename).expect("Failed to save file");
    std::process::Command::new("sxiv")
        .arg(filename)
        .output()
        .expect("Failed to execute sxiv");
}
