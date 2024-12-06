use crate::draw::{Drawable, Size};
use crate::game::Direction;
use crate::objects::enemy::Enemy;
use crate::objects::starship::Starship;
use piston_window::{Context, G2d};
use std::process;

pub struct EnemiesConfig {
    number: u32,
    width: u32,
    space_between: u32,
    layers: u32,
}

const LAYERS: u32 = 5;
const SPACE_BETWEEN_ENEMIES: u32 = 30;

impl EnemiesConfig {
    pub fn new(number: u32, width: u32) -> Self {
        Self {
            number,
            width,
            space_between: SPACE_BETWEEN_ENEMIES,
            layers: LAYERS,
        }
    }
}

pub struct EnemyManager {
    enemies: Vec<Enemy>,
    direction: Direction,
}

impl EnemyManager {
    pub fn new(config: EnemiesConfig, window_width: u32) -> Self {
        let enemies = Self::generate_enemies(&config, window_width);

        Self {
            enemies,
            direction: Direction::Right,
        }
    }
    fn generate_enemies(config: &EnemiesConfig, window_width: u32) -> Vec<Enemy> {
        let total_enemies = config.layers * config.number;
        let height = 20;
        let total_window_width = (config.width + config.space_between) * config.number;

        if total_window_width > window_width {
            eprintln!("The number of the enemies exceeds the screen size.");
            process::exit(1);
        }

        let initial_position = (window_width - total_window_width) / 2;

        // Use from_iterator for better performance
        (0..total_enemies)
            .map(|i| {
                let row = (i / config.number) + 1;
                let position_x =
                    initial_position + (config.width + config.space_between) * (i % config.number);
                let position_y = row * (height + config.space_between);
                Enemy::new(config.width, height, position_x, position_y)
            })
            .collect()
    }

    pub fn update(&mut self, window_size: &Size) {
        let mut move_down = false;

        for enemy in &self.enemies {
            let enemy_position = enemy.get_position();
            let enemy_size = enemy.get_size();

            if enemy_position.x + enemy_size.width >= window_size.width || enemy_position.x < 1 {
                self.direction = self.direction.opposite();
                move_down = true;
                break;
            }
        }

        self.enemies.iter_mut().for_each(|enemy| {
            let position = enemy.get_position();
            let new_y = position.y + if move_down { 10 } else { 0 };
            let new_x = match self.direction {
                Direction::Left => position.x - 1,
                Direction::Right => position.x + 1,
            };
            enemy.update(new_x, new_y);
        });
    }

    pub fn retain_enemies<F>(&mut self, mut retain_fn: F)
    where
        F: FnMut(&Enemy) -> bool,
    {
        self.enemies.retain(|enemy| retain_fn(enemy));
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        self.enemies
            .iter()
            .for_each(|enemy| enemy.draw(context, graphics));
    }

    pub fn check_collision_with_starship(&self, starship: &Starship) -> bool {
        self.enemies
            .iter()
            .any(|enemy| starship.check_collision(enemy))
    }

    pub fn remaining_enemies(&self) -> usize {
        self.enemies.len()
    }
}
