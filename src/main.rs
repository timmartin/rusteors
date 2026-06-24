use macroquad::prelude::*;
use rusteors::World;
use rusteors::load_textures;
use simple_logger::SimpleLogger;

#[macroquad::main("Rusteors")]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let textures = load_textures().await.unwrap();

    let mut world = World::new(textures);

    loop {
        clear_background(BLACK);

        world.update();
        world.draw();

        next_frame().await
    }
}
