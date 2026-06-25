use macroquad::prelude::*;

/// Call a draw function, wrapping it round the screen (calling multiple times if
/// necessary to simulate the drawn object wrapping)
///
/// # Arguments
///
/// * `bounding_size` - The width of a bounding box, measured from the middle
///     point, that will contain the element being drawn.
pub fn draw_wrapped(position: Vec2, bounding_size: f32, draw: impl Fn(Vec2)) {
    let w = screen_width();
    let h = screen_height();

    for dx in [0.0]
        .into_iter()
        .chain((position.x - bounding_size < 0.0).then_some(w))
        .chain((position.x + bounding_size > w).then_some(-w))
    {
        for dy in [0.0]
            .into_iter()
            .chain((position.y - bounding_size < 0.0).then_some(h))
            .chain((position.y + bounding_size > h).then_some(-h))
        {
            draw(Vec2::new(position.x + dx, position.y + dy));
        }
    }
}

/// Draw a texture on the screen with wrapping, centered on the given position.
pub fn draw_wrapped_texture(position: Vec2, radius: f32, texture: &Texture2D) {
    let size = radius * 2.0;
    let draw_texture_inner = |draw_position: Vec2| {
        draw_texture_ex(
            texture,
            draw_position.x - radius,
            draw_position.y - radius,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    };

    draw_wrapped(position, radius, draw_texture_inner);
}

/// Draw a circle on the screen based on position in a space that wraps
/// around in both x and y directions.
pub fn draw_wrapped_circle(position: Vec2, radius: f32, color: Color) {
    let draw_circle_inner = |draw_position: Vec2| {
        draw_circle(draw_position.x, draw_position.y, radius, color);
    };

    draw_wrapped(position, radius, draw_circle_inner);
}

/// Draw a triangle on the screen with wrapping, using the center position for
/// edge detection.
pub fn draw_wrapped_triangle(
    position: Vec2,
    radius: f32,
    v1: Vec2,
    v2: Vec2,
    v3: Vec2,
    color: Color,
) {
    let draw_triangle_inner = |draw_position: Vec2| {
        draw_triangle(
            draw_position + (v1 - position),
            draw_position + (v2 - position),
            draw_position + (v3 - position),
            color,
        );
    };

    draw_wrapped(position, radius, draw_triangle_inner);
}
