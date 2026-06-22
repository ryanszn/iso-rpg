#![allow(dead_code)]
use macroquad::prelude::*;

mod constants;
mod drawing;
mod game;
mod math;
mod pathfinding;
mod types;

use game::Game;
use types::AppState;

#[macroquad::main("Roguey")]
async fn main() {
    let mut game = Game::new();
    let mut state = AppState::Menu;

    loop {
        clear_background(WHITE);

        match state {
            AppState::Menu => {
                draw_text("Menu - Enter to start", 100., 100., 40., BLACK);
                if is_key_pressed(KeyCode::Enter) {
                    game = Game::new();
                    state = AppState::Playing;
                }
            }
            AppState::Playing => {
                if game.update(get_frame_time()) {
                    state = AppState::GameOver;
                }
                game.draw();
            }
            AppState::GameOver => {
                game.draw();
                draw_rectangle(
                    0.,
                    0.,
                    screen_width(),
                    screen_height(),
                    Color::new(1., 1., 1., 0.7),
                );
                // Victory vs Defeat Logic
                let (msg, col) = if game.hp > 0 {
                    ("VICTORY", GOLD)
                } else {
                    ("GAME OVER", RED)
                };

                draw_text(
                    msg,
                    screen_width() / 2. - 100.,
                    screen_height() / 2.,
                    60.,
                    col,
                );
                draw_text(
                    &format!("Final Score: {}", game.score),
                    screen_width() / 2.,
                    screen_height() / 2. + 50.,
                    30.,
                    BLACK,
                );
                draw_text(
                    "Enter to reset",
                    screen_width() / 2. - 80.,
                    screen_height() / 2. + 90.,
                    20.,
                    GRAY,
                );

                if is_key_pressed(KeyCode::Enter) {
                    state = AppState::Menu;
                }
            }
        }
        next_frame().await;
    }
}
