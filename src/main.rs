use macroquad::prelude::*;
use simple_logger::SimpleLogger;

mod meteor;

const RADIUS: f32 = 25.0;

#[macroquad::main("Rusteors")]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let mut meteors = [
        meteor::Meteor::new(
            Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
            RADIUS,
            BLUE,
            Vec2::new(160.0, -40.0),
        ),
        meteor::Meteor::new(
            Vec2::new(screen_width() / 4.0, screen_height() / 4.0),
            RADIUS * 0.8,
            RED,
            Vec2::new(-120.0, 80.0),
        ),
        meteor::Meteor::new(
            Vec2::new(screen_width() * 3.0 / 4.0, screen_height() * 3.0 / 4.0),
            RADIUS * 1.2,
            GREEN,
            Vec2::new(60.0, 100.0),
        ),
    ];

    loop {
        clear_background(BLACK);

        for meteor in &mut meteors {
            meteor.update();
        }

        for metor in &meteors {
            metor.draw();
        }

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
