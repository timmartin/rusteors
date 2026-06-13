use macroquad::prelude::*;

const SPEED: f32 = 150.0;
const RADIUS: f32 = 25.0;

#[macroquad::main("Rusteors")]
async fn main() {
    let mut x = RADIUS;
    let y = screen_height() / 2.0;

    loop {
        x = (x + SPEED * get_frame_time()) % screen_width();

        clear_background(BLACK);
        draw_wrapped_circle(x, y, RADIUS, BLUE);

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

