use piston_window::{clear, PistonWindow, WindowSettings};

use space_invaders::draw::Size;
use space_invaders::Game;

fn draw_window(title: &str, size: &Size) -> PistonWindow {
    WindowSettings::new(title, [size.width, size.height])
        .exit_on_esc(true)
        .build()
        .unwrap()
}

fn main() {
    let mut game = Game::new(840, 600);
    let mut window = draw_window("Space Invaders", game.get_window_size());

    let font_path = std::path::Path::new("assets/Jersey10-Regular.ttf");
    let mut glyphs = window.load_font(&font_path).unwrap();

    while let Some(e) = window.next() {
        game.handle_event(&e);
        game.update();

        window.draw_2d(&e, |c, g, _device| {
            clear(game.get_color(), g);
            game.draw(&c, g, &mut glyphs);

            // https://github.com/PistonDevelopers/piston_window/issues/258
            glyphs.factory.encoder.flush(_device);
        });
    }
}
