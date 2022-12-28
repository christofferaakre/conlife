#[derive(Debug, Clone, Copy)]
pub struct Cell {
    alive: bool,
}

impl Cell {
    pub fn new() -> Self {
        Self { alive: false }
    }
}

#[derive(Debug)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        let mut cells = vec![];
        let cell = Cell::new();
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                row.push(cell)
            }
            cells.push(row);
        }
        Self {
            width,
            height,
            cells,
        }
    }
}

const WIDTH: u32 = 64;
const HEIGHT: u32 = 64;

fn main() {
    let grid = Grid::new(WIDTH, HEIGHT);
    // println!("{:?}", grid);

    use image::{Rgb, RgbImage};

    let mut image = RgbImage::new(WIDTH, HEIGHT);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            image.put_pixel(x, y, Rgb([255, 0, 0]));
        }
    }

    let filename = "out/out.png";
    image.save(filename).expect("Failed to save file");
    std::process::Command::new("sxiv")
        .arg(filename)
        .output()
        .expect("Failed to execute sxiv");
}
