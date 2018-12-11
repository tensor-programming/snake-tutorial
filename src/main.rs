extern crate piston_window;
extern crate rand;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use draw::to_coord_u32;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

const WIDTH: i32 = 30;
const HEIGHT: i32 = 36;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(WIDTH), to_coord_u32(HEIGHT)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(WIDTH, HEIGHT);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
