use macroquad::prelude::*;
use rusteors::World;
use simple_logger::SimpleLogger;

#[macroquad::main("Rusteors")]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let mut world = World::new();

    loop {
        clear_background(BLACK);

        world.update();
        world.draw();

        next_frame().await
    }
}
