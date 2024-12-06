use piston_window::{Context, G2d};

use crate::draw::{Drawable, Position};
use crate::objects::bullet::{Bullet, BulletConfig};
use crate::objects::starship::Starship;

pub struct BulletManager {
    bullets: Vec<Bullet>,
    config: BulletConfig,
}

impl BulletManager {
    pub fn new(config: BulletConfig) -> Self {
        Self {
            bullets: Vec::new(),
            config,
        }
    }

    pub fn fire(&mut self, starship: &Starship) {
        let bullet_position = self.get_bullet_start_position(starship);
        let bullet = Bullet::new(bullet_position, self.config.clone());
        self.bullets.push(bullet);
    }

    fn get_bullet_start_position(&self, starship: &Starship) -> Position {
        let starship_position = starship.get_position();
        let starship_width = starship.get_size().width;
        let bullet_width = self.config.get_size().width;

        Position::set(
            starship_position.x + (starship_width / 2) - (bullet_width / 2),
            starship_position.y,
        )
    }

    pub fn update(&mut self) {
        self.bullets.retain_mut(|bullet| {
            bullet.update();
            !bullet.is_off_screen()
        });
    }

    pub fn check_collisions<F>(&mut self, mut collision_fn: F)
    where
        F: FnMut(&Bullet) -> bool,
    {
        self.bullets.retain(|bullet| !collision_fn(bullet));
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        self.bullets
            .iter()
            .for_each(|bullet| bullet.draw(context, graphics));
    }
}
