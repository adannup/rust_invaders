use piston_window::input::{Key, PressEvent};
use piston_window::types::Color;
use piston_window::{Button, CharacterCache, Context, Event, G2d, Glyphs};

use crate::draw::{self, Drawable, Position, Size};
use crate::input_manager::InputManager;
use crate::managers::bullets::BulletManager;
use crate::managers::enemies::{EnemiesConfig, EnemyManager};
use crate::objects::bullet::BulletConfig;
use crate::objects::starship::Starship;

pub enum KeyboardEvents {
    PressEvent,
    ReleaseEvent,
}

pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

const BACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const TEXT_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

pub enum GameState {
    Win,
    Playing,
    GameOver,
}

pub struct Game {
    state: GameState,
    window_size: Size,
    color: Color,
    starship: Starship,
    input_manager: InputManager,
    bullets_manager: BulletManager,
    enemies_manager: EnemyManager,
    score: i32,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        let window_size = Size::set(height, width);
        let starship = Starship::config(&window_size, 20, 50);
        let bullet_config = BulletConfig::new(2, 10);
        let enemies_config = EnemiesConfig::new(8, 50);

        Self {
            window_size,
            color: BACK_COLOR,
            starship,
            input_manager: InputManager::new(),
            bullets_manager: BulletManager::new(bullet_config),
            enemies_manager: EnemyManager::new(enemies_config, width),
            score: 0,
            state: GameState::Playing,
        }
    }

    pub fn get_window_size(&self) -> &Size {
        &self.window_size
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    fn get_score(&self) -> i32 {
        self.score
    }

    fn update_score(&mut self, new_score: i32) {
        self.score = new_score;
    }

    pub fn draw(&mut self, context: &Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
        match self.state {
            GameState::Playing => self.draw_gameplay(context, graphics, glyphs),
            GameState::GameOver => self.draw_text_screen("Game Over", context, graphics, glyphs),
            GameState::Win => {
                self.draw_score(context, graphics, glyphs);
                self.draw_text_screen("You Win!", context, graphics, glyphs);
            }
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        self.input_manager.handle_event(event);

        if let Some(Button::Keyboard(key)) = event.press_args() {
            if key == Key::Space {
                self.bullets_manager.fire(&self.starship);
            }
        }
    }

    fn draw_gameplay(&mut self, context: &Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
        self.draw_score(context, graphics, glyphs);
        self.bullets_manager.draw(context, graphics);
        self.enemies_manager.draw(context, graphics);
        self.starship.draw(context, graphics);
    }

    fn draw_text_screen(
        &self,
        text: &str,
        context: &Context,
        graphics: &mut G2d,
        glyphs: &mut Glyphs,
    ) {
        let font_size = 40;
        let text = text;
        let text_width = glyphs.width(font_size, text).unwrap() as u32;

        draw::draw_text(
            &context,
            graphics,
            glyphs,
            TEXT_COLOR,
            &Position {
                x: (self.get_window_size().width - text_width) / 2,
                y: self.get_window_size().height / 2,
            },
            text,
            font_size,
        );
    }

    fn draw_score(&self, context: &Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
        let font_size = 24;
        let text = format!("score {}", &self.get_score());
        let text_width = glyphs.width(font_size, &text).unwrap() as u32;
        let margin_size = 40;

        draw::draw_text(
            &context,
            graphics,
            glyphs,
            TEXT_COLOR,
            &Position {
                x: (self.get_window_size().width - text_width - margin_size),
                y: margin_size,
            },
            &text,
            font_size,
        );
    }

    fn handle_starship_movement(&mut self) {
        let screen_width = self.window_size.width;

        if self.input_manager.is_key_pressed(Key::Left) {
            self.starship.move_starship(Direction::Left, screen_width);
        }
        if self.input_manager.is_key_pressed(Key::Right) {
            self.starship.move_starship(Direction::Right, screen_width);
        }
    }

    fn handle_bullet_collisions(&mut self) {
        let mut score_to_add = self.score;
        let enemies_manager = &mut self.enemies_manager;
        self.bullets_manager.check_collisions(|bullet| {
            let mut hit = false;
            enemies_manager.retain_enemies(|enemy| {
                if enemy.is_hit(bullet.get_size(), &bullet.get_position()) {
                    score_to_add += 100;
                    hit = true;
                    false
                } else {
                    true
                }
            });
            hit
        });

        self.update_score(score_to_add);
    }

    fn handle_win(&mut self) {
        if self.enemies_manager.remaining_enemies() == 0 {
            self.state = GameState::Win;
        }
    }

    pub fn update(&mut self) {
        self.handle_starship_movement();
        self.enemies_manager.update(&self.window_size);
        self.bullets_manager.update();

        if self
            .enemies_manager
            .check_collision_with_starship(&self.starship)
        {
            self.state = GameState::GameOver;
        }

        self.handle_bullet_collisions();
        self.handle_win();
    }
}
