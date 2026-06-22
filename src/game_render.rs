use macroquad::prelude::*;

use crate::constants::MAP_SIZE;
use crate::drawing::{draw_stickman, draw_wall};
use crate::game::Game;
use crate::math::to_screen;
use crate::types::Tile;

impl Game {
    pub fn draw_ui(&self) {
        draw_text(
            &format!("HP: {}", self.hp),
            20.0,
            screen_height() - 40.0,
            30.0,
            BLACK,
        );

        draw_text(
            &format!("SCORE: {}", self.score),
            20.0,
            screen_height() - 70.0,
            30.0,
            BLACK,
        );
    }

    pub fn draw_floating_text(&self) {
        for text in &self.texts {
            if text.dmg < 0 {
                draw_text(&format!("+{}", -text.dmg), text.x, text.y, 20.0, text.color);
            } else {
                draw_text(&format!("-{}", text.dmg), text.x, text.y, 20.0, text.color);
            }
        }
    }

    pub fn draw_entities(&self) {
        draw_stickman(self.player_x, self.player_y, self.camera, false);

        for monster in &self.monsters {
            draw_stickman(monster.x, monster.y, self.camera, true);
        }
    }

    pub fn draw_path(&self) {
        for (path_x, path_y) in &self.path {
            let (screen_x, screen_y) = to_screen(*path_x, *path_y, self.camera);
            draw_circle(screen_x, screen_y + 16.0, 4.0, GOLD);
        }
    }

    pub fn draw_world(&self) {
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                if self.map[y][x] == Tile::Wall {
                    draw_wall(x, y, self.camera);
                } else if self.potions.contains(&(x, y)) {
                    let (screen_x, screen_y) = to_screen(x, y, self.camera);
                    draw_circle(screen_x, screen_y + 8.0, 14.0, RED);
                } else if self.gold.contains(&(x, y)) {
                    let (screen_x, screen_y) = to_screen(x, y, self.camera);
                    draw_circle(screen_x, screen_y, 16.0, GOLD);
                } else {
                    let (screen_x, screen_y) = to_screen(x, y, self.camera);
                    draw_circle(screen_x, screen_y + 16.0, 2.0, LIGHTGRAY);
                }
            }
        }
    }
}
