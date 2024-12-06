use piston_window::types::Color;
use piston_window::{Context, G2d};

use crate::draw::{self, Drawable, Position, Size};

#[derive(Debug)]
pub struct Enemy {
    color: Color,
    position: Position,
    size: Size,
}

impl Drawable for Enemy {
    fn draw(&self, con: &Context, g: &mut G2d) {
        draw::draw_block(con, g, self.color, &self.position, &self.size);
    }

    fn get_position(&self) -> &Position {
        &self.position
    }

    fn get_size(&self) -> &Size {
        &self.size
    }
}

impl Enemy {
    pub fn new(width: u32, height: u32, x: u32, y: u32) -> Self {
        Self {
            position: Position::set(x, y),
            color: [1.0, 1.0, 1.0, 1.0],
            size: Size::set(height, width),
        }
    }

    pub fn is_hit(&self, bullet_size: &Size, bullet_position: &Position) -> bool {
        bullet_position.x + bullet_size.width >= self.position.x
            && bullet_position.x <= self.position.x + self.size.width
            && bullet_position.y + bullet_size.height >= self.position.y
            && bullet_position.y + bullet_size.height <= self.position.y + self.size.height
    }

    pub fn update(&mut self, x: u32, y: u32) {
        self.position.x = x;
        self.position.y = y;
    }
}
