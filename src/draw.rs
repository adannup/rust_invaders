use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d, Glyphs, Text, Transformed};

#[derive(Debug)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Debug)]
pub struct Size {
    pub height: u32,
    pub width: u32,
}

impl Size {
    pub fn set(height: u32, width: u32) -> Self {
        Size { height, width }
    }
}

impl Position {
    pub fn set(x: u32, y: u32) -> Self {
        Position { x, y }
    }
}

pub trait Drawable {
    fn get_size(&self) -> &Size;
    fn draw(&self, con: &Context, g: &mut G2d);
    fn get_position(&self) -> &Position;
}

fn to_f64(value: u32) -> f64 {
    value as f64
}

pub fn draw_block(con: &Context, g: &mut G2d, color: Color, position: &Position, size: &Size) {
    let position_x = to_f64(position.x);
    let position_y = to_f64(position.y);
    let width = to_f64(size.width);
    let height = to_f64(size.height);

    rectangle(
        color,
        [position_x, position_y, width, height],
        con.transform,
        g,
    )
}

pub fn draw_text(
    con: &Context,
    g: &mut G2d,
    glyphs: &mut Glyphs,
    color: Color,
    position: &Position,
    text: &str,
    font_size: u32,
) {
    let position_x = to_f64(position.x);
    let position_y = to_f64(position.y);

    let transform = con.transform.trans(position_x, position_y);

    Text::new_color(color, font_size)
        .draw(text, glyphs, &con.draw_state, transform, g)
        .unwrap();
}
