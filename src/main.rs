use macroquad::prelude::*;
use log::info;
use simple_logger::SimpleLogger;

mod meteor;

const RADIUS: f32 = 25.0;

#[macroquad::main("Rusteors")]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let start_position = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);

    let mut meteor = meteor::Meteor::new(start_position, RADIUS, BLUE, Vec2::new(160.0, -40.0));

    loop {
        clear_background(BLACK);

        meteor.update();
        meteor.draw();

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
