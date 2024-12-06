use piston_window::types::Color;
use piston_window::{Context, G2d};

use crate::draw::{self, Drawable, Position, Size};
use crate::game::Direction;

use super::enemy::Enemy;

pub struct Starship {
    size: Size,
    color: Color,
    position: Position,
    move_speed: u32,
}

impl Drawable for Starship {
    fn get_position(&self) -> &Position {
        &self.position
    }

    fn get_size(&self) -> &Size {
        &self.size
    }

    fn draw(&self, con: &Context, g: &mut G2d) {
        draw::draw_block(con, g, self.color, &self.position, &self.size);
    }
}

impl Starship {
    pub fn config(window_size: &Size, height: u32, width: u32) -> Self {
        let size = Size::set(height, width);
        let position_y = window_size.height - size.height - 30;
        let position_x = (window_size.width - size.width) / 2;

        Self {
            size,
            color: [1.0, 1.0, 1.0, 1.0],
            position: Position::set(position_x, position_y),
            move_speed: 1,
        }
    }

    pub fn move_starship(&mut self, direction: Direction, screen_width: u32) {
        match direction {
            Direction::Left => self.position.x -= &self.move_speed,
            Direction::Right => self.position.x += &self.move_speed,
        }

        self.position.x = self.position.x.clamp(1, screen_width - self.size.width);
    }

    pub fn check_collision(&self, enemy: &Enemy) -> bool {
        let starship_position = self.get_position();
        let enemy_position = enemy.get_position();
        let enemy_size = enemy.get_size();

        starship_position.y < enemy_position.y + enemy_size.height
    }
}
