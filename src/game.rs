use macroquad::prelude::*;

use crate::constants::MAP;
use crate::drawing::{draw_stickman, draw_wall};
use crate::math::{dist, to_screen, to_tile};
use crate::pathfinding::bfs;
use crate::types::{DmgText, Monster, Tile};

pub struct Game {
    pub map: [[Tile; MAP]; MAP],
    pub cam: (f32, f32),
    pub px: usize,
    pub py: usize,
    pub path: Vec<(usize, usize)>,
    pub player_cd: f32,
    pub monsters: Vec<Monster>,
    pub texts: Vec<DmgText>,
    pub hp: i32,
    pub gold: Vec<(usize, usize)>,
    pub score: i32,
}

impl Game {
    pub fn new() -> Self {
        let mut map = [[Tile::Floor; MAP]; MAP];

        for i in 0..MAP {
            map[0][i] = Tile::Wall;
            map[MAP - 1][i] = Tile::Wall;
            map[i][0] = Tile::Wall;
            map[i][MAP - 1] = Tile::Wall;
        }

        // Add obstacles
        for (x, y) in [(5, 5), (6, 5), (12, 10)] {
            map[y][x] = Tile::Wall;
        }

        Game {
            map,
            cam: (screen_width() / 2., 50.),
            px: 2,
            py: 2,
            path: vec![],
            player_cd: 0.,
            monsters: vec![
                Monster {
                    x: 8,
                    y: 8,
                    hp: 30,
                    cd: 0.,
                },
                Monster {
                    x: 12,
                    y: 4,
                    hp: 30,
                    cd: 0.,
                },
                Monster {
                    x: 15,
                    y: 12,
                    hp: 30,
                    cd: 0.,
                },
            ],
            texts: vec![],
            hp: 100,
            score: 0,
            gold: vec![(3, 3), (10, 2), (16, 5), (6, 14), (17, 17)],
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        if self.hp <= 0 || self.monsters.is_empty() {
            return true;
        }

        // Update text animations
        self.texts.retain_mut(|t| {
            t.life -= dt;
            t.y -= 20. * dt;
            t.life > 0.
        });

        // Mouse input logic
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let (tx, ty) = to_tile(mx, my, self.cam);

            if tx < MAP && ty < MAP && self.map[ty][tx] == Tile::Floor {
                self.path = bfs(&self.map, (self.px, self.py), (tx, ty));
            }
        }

        // Handle movement for player
        if !self.path.is_empty() {
            self.player_cd -= dt;

            if self.player_cd <= 0. {
                self.player_cd = 0.15;

                let (nx, ny) = self.path[0];

                if let Some(i) = self.monsters.iter().position(|m| m.x == nx && m.y == ny) {
                    self.damage_monster(i, 10);
                    self.path.clear();
                } else {
                    self.path.remove(0);
                    self.px = nx;
                    self.py = ny;

                    if let Some(i) = self.gold.iter().position(|&g| g == (self.px, self.py)) {
                        self.gold.remove(i);
                        self.score += 100;

                        let (sx, sy) = to_screen(self.px, self.py, self.cam);
                        self.texts.push(DmgText {
                            x: sx,
                            y: sy - 40.,
                            dmg: -100,
                            life: 1.,
                        });
                    }
                }
            }
        }

        let occupied: Vec<_> = self
            .monsters
            .iter()
            .map(|m| (m.x, m.y))
            .chain(std::iter::once((self.px, self.py)))
            .collect();

        for i in 0..self.monsters.len() {
            self.monsters[i].cd -= dt;

            if self.monsters[i].cd <= 0. {
                self.monsters[i].cd = 1.0;

                let (mx, my) = (self.monsters[i].x, self.monsters[i].y);
                let d = dist((mx, my), (self.px, self.py));

                if d == 1 {
                    self.hp -= 5;

                    // small fix: this was self.py, self.py before
                    let (sx, sy) = to_screen(self.px, self.py, self.cam);

                    self.texts.push(DmgText {
                        x: sx,
                        y: sy,
                        dmg: 5,
                        life: 1.,
                    });
                } else {
                    let path = bfs(&self.map, (mx, my), (self.px, self.py));

                    if path.len() > 1 && !occupied.contains(&path[0]) {
                        self.monsters[i].x = path[0].0;
                        self.monsters[i].y = path[0].1;
                    }
                }
            }
        }

        false
    }

    pub fn damage_monster(&mut self, idx: usize, amount: i32) {
        self.monsters[idx].hp -= amount;

        let (sx, sy) = to_screen(self.monsters[idx].x, self.monsters[idx].y, self.cam);

        self.texts.push(DmgText {
            x: sx,
            y: sy - 40.,
            dmg: amount,
            life: 1.,
        });

        if self.monsters[idx].hp <= 0 {
            self.monsters.remove(idx);
            self.score += 50;
        }
    }

    pub fn draw(&self) {
        for y in 0..MAP {
            for x in 0..MAP {
                if self.map[y][x] == Tile::Wall {
                    draw_wall(x, y, self.cam);
                } else if self.gold.contains(&(x, y)) {
                    let (sx, sy) = to_screen(x, y, self.cam);
                    draw_circle(sx, sy, 16., GOLD);
                } else {
                    let (sx, sy) = to_screen(x, y, self.cam);
                    draw_circle(sx, sy + 16., 2., LIGHTGRAY);
                }
            }
        }

        for (px, py) in &self.path {
            let (sx, sy) = to_screen(*px, *py, self.cam);
            draw_circle(sx, sy + 16., 4., GOLD);
        }

        draw_stickman(self.px, self.py, self.cam, false);

        for m in &self.monsters {
            draw_stickman(m.x, m.y, self.cam, true);
        }

        for t in &self.texts {
            if t.dmg < 0 {
                draw_text(&format!("+{}", -t.dmg), t.x, t.y, 20., GREEN);
            } else {
                draw_text(&format!("-{}", t.dmg), t.x, t.y, 20., RED);
            }
        }

        draw_text(
            &format!("HP: {}", self.hp),
            20.,
            screen_height() - 40.,
            30.,
            BLACK,
        );

        draw_text(
            &format!("SCORE: {}", self.score),
            20.,
            screen_height() - 70.,
            30.,
            BLACK,
        );
    }
}
