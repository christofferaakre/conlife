use life::{Grid, Object};
use open_oak::{
    init::{init, Game},
    resource_manager::ResourceManager,
    shapes::rect::Rectangle,
    traits::Shaders,
};

use open_oak::{Rad, Rgba, Vector2};

fn main() {
    let mut grid = Grid::new(16, 16);

    let glider = Object::from_file("objects/glider.life");
    grid.load_object(&glider, (0, 0));
    grid.advance();

    let game = init();

    let Game {
        display,
        event_loop,
        mut resource_manager,
        ..
    } = game;

    // init rectangle
    Rectangle::init(&mut resource_manager, &display);

    let texture_name = String::from("cell");
    let texture = ResourceManager::load_texture(&display, "textures/cell.png");
    resource_manager.add_texture(&texture_name, texture);

    let rect = Rectangle::new(
        Vector2::new(0.0, 0.0),
        Vector2::new(0.2, 0.2),
        Rad(0.0),
        Rgba([1.0, 0.0, 0.0, 1.0]),
    );
}
