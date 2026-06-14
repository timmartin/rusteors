use macroquad::prelude::*;
use simple_logger::SimpleLogger;

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

/// Draw a circle on the screen based on position in a space that wraps
/// around in both x and y directions.
fn draw_wrapped_circle(x: f32, y: f32, radius: f32, color: Color) {
    let w = screen_width();
    let h = screen_height();

    for dx in [0.0]
        .into_iter()
        .chain((x - radius < 0.0).then_some(w))
        .chain((x + radius > w).then_some(-w))
    {
        for dy in [0.0]
            .into_iter()
            .chain((y - radius < 0.0).then_some(h))
            .chain((y + radius > h).then_some(-h))
        {
            draw_circle(x + dx, y + dy, radius, color);
        }
    }
}

/// Draw a triangle on the screen with wrapping, using the center position for
/// edge detection.
fn draw_wrapped_triangle(
    x: f32,
    y: f32,
    radius: f32,
    v1: Vec2,
    v2: Vec2,
    v3: Vec2,
    color: Color,
) {
    let w = screen_width();
    let h = screen_height();

    for dx in [0.0]
        .into_iter()
        .chain((x - radius < 0.0).then_some(w))
        .chain((x + radius > w).then_some(-w))
    {
        for dy in [0.0]
            .into_iter()
            .chain((y - radius < 0.0).then_some(h))
            .chain((y + radius > h).then_some(-h))
        {
            let offset = Vec2::new(dx, dy);
            draw_triangle(v1 + offset, v2 + offset, v3 + offset, color);
        }
    }
}
