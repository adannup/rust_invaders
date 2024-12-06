use crate::draw::{self, Drawable, Position, Size};
use piston_window::types::Color;
use piston_window::{Context, G2d};

#[derive(Clone)]
pub struct BulletConfig {
    speed: u32,
    size: Size,
}

impl BulletConfig {
    pub fn new(speed: u32, size: u32) -> Self {
        Self {
            speed,
            size: Size::set(size, size),
        }
    }

    pub fn get_size(&self) -> &Size {
        &self.size
    }
}

pub struct Bullet {
    position: Position,
    color: Color,
    config: BulletConfig,
}

impl Drawable for Bullet {
    fn get_size(&self) -> &Size {
        &self.config.size
    }

    fn draw(&self, con: &Context, g: &mut G2d) {
        draw::draw_block(con, g, self.color, &self.position, &self.config.size);
    }

    fn get_position(&self) -> &Position {
        &self.position
    }
}

impl Bullet {
    pub fn new(position: Position, config: BulletConfig) -> Self {
        Self {
            position,
            config,
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    pub fn is_off_screen(&self) -> bool {
        self.position.y < 1
    }

    pub fn update(&mut self) {
        self.position.y -= self.config.speed;
    }
}
