use std::time::Instant;

use life::{Grid, Object};
use open_oak::{
    init::{init, Game},
    resource_manager::ResourceManager,
    shapes::rect::Rectangle,
    traits::{Renderable, Shaders, Texture},
};

use open_oak::Surface;
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

    let mut rect = Rectangle::new(
        Vector2::new(0.1, 0.1),
        Vector2::new(0.2, 0.2),
        Rad(0.0),
        Rgba([1.0, 0.0, 0.0, 1.0]),
    );

    rect.set_texture(texture_name);

    let mut last_frame = Instant::now();
    // game loop
    event_loop.run(move |ev, _, _control_flow| {
        // calculate time since last frame
        let dt = last_frame.elapsed();
        last_frame += dt;

        let mut frame = display.draw();
        frame.clear_color(0.2, 0.3, 0.3, 1.0);

        // DRAW START
        rect.draw(&mut frame, &resource_manager).unwrap();

        frame.finish().unwrap();
        // DRAW END
    });
}
