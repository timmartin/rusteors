use macroquad::prelude::*;
use simple_logger::SimpleLogger;

mod draw;
mod meteor;
mod ship;
mod world;

#[macroquad::main("Rusteors")]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let mut world = world::World::new();

    loop {
        clear_background(BLACK);

        world.update();
        world.draw();

        next_frame().await
    }
}
