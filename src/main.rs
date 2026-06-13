use macroquad::prelude::*;

mod meteor;

const SPEED: f32 = 160.0;
const RADIUS: f32 = 25.0;

#[macroquad::main("Rusteors")]
async fn main() {
    let mut meteor = meteor::Meteor::new(screen_width() / 2.0, screen_height() / 2.0, RADIUS, BLUE, SPEED);

    loop {
        clear_background(BLACK);

        meteor.update();
        meteor.draw();

        next_frame().await
    }
}

fn draw_wrapped_circle(x: f32, y: f32, radius: f32, color: Color) {
    let w = screen_width();

    if x - radius < 0.0 {
        draw_circle(x + w, y, radius, color);
    } else if x + radius > w {
        draw_circle(x - w, y, radius, color);
    }
    
    draw_circle(x, y, radius, color);
}

