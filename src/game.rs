use macroquad::prelude::*;

use crate::constants::MAP_SIZE;
use crate::drawing::{draw_stickman, draw_wall};
use crate::math::{dist, to_screen, to_tile};
use crate::pathfinding::bfs;
use crate::types::{DmgText, Monster, ScreenPos, Tile, TilePos};

pub struct Game {
    pub map: [[Tile; MAP_SIZE]; MAP_SIZE],
    pub camera: ScreenPos,
    pub player_x: usize,
    pub player_y: usize,
    pub path: Vec<TilePos>,
    pub player_cooldown: f32,
    pub monsters: Vec<Monster>,
    pub texts: Vec<DmgText>,
    pub hp: i32,
    pub gold: Vec<TilePos>,
    pub potions: Vec<TilePos>,
    pub score: i32,
}

impl Game {
    pub fn new() -> Self {
        let mut map = [[Tile::Floor; MAP_SIZE]; MAP_SIZE];

        for i in 0..MAP_SIZE {
            map[0][i] = Tile::Wall;
            map[MAP_SIZE - 1][i] = Tile::Wall;
            map[i][0] = Tile::Wall;
            map[i][MAP_SIZE - 1] = Tile::Wall;
        }

        // Add obstacles
        for (x, y) in [(5, 5), (6, 5), (12, 10)] {
            map[y][x] = Tile::Wall;
        }

        Game {
            map,
            camera: (screen_width() / 2.0, 50.0),
            player_x: 2,
            player_y: 2,
            path: vec![],
            player_cooldown: 0.0,
            monsters: vec![
                Monster {
                    x: 8,
                    y: 8,
                    hp: 30,
                    cooldown: 0.0,
                },
                Monster {
                    x: 12,
                    y: 4,
                    hp: 30,
                    cooldown: 0.0,
                },
                Monster {
                    x: 15,
                    y: 12,
                    hp: 30,
                    cooldown: 0.0,
                },
            ],
            texts: vec![],
            hp: 100,
            score: 0,
            gold: vec![(3, 3), (10, 2), (16, 5), (6, 14), (17, 17)],
            potions: vec![(3, 2)],
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        if self.hp <= 0 || self.monsters.is_empty() {
            return true;
        }

        // Update text animations
        self.texts.retain_mut(|text| {
            text.life -= dt;
            text.y -= 20.0 * dt;
            text.life > 0.0
        });

        // Mouse input logic
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let (target_x, target_y) = to_tile(mouse_x, mouse_y, self.camera);

            if target_x < MAP_SIZE
                && target_y < MAP_SIZE
                && self.map[target_y][target_x] == Tile::Floor
            {
                self.path = bfs(
                    &self.map,
                    (self.player_x, self.player_y),
                    (target_x, target_y),
                );
            }
        }

        // Handle movement for player
        if !self.path.is_empty() {
            self.player_cooldown -= dt;

            if self.player_cooldown <= 0.0 {
                self.player_cooldown = 0.15;

                let (next_x, next_y) = self.path[0];

                if let Some(monster_index) = self
                    .monsters
                    .iter()
                    .position(|monster| monster.x == next_x && monster.y == next_y)
                {
                    self.damage_monster(monster_index, 10);
                    self.path.clear();
                } else {
                    self.path.remove(0);
                    self.player_x = next_x;
                    self.player_y = next_y;

                    self.collect_potion();

                    if let Some(gold_index) = self
                        .gold
                        .iter()
                        .position(|&gold| gold == (self.player_x, self.player_y))
                    {
                        self.gold.remove(gold_index);
                        self.score += 100;

                        let (screen_x, screen_y) =
                            to_screen(self.player_x, self.player_y, self.camera);

                        self.texts.push(DmgText {
                            x: screen_x,
                            y: screen_y - 40.0,
                            dmg: -100,
                            life: 1.0,
                            color: GOLD,
                        });
                    }
                }
            }
        }

        let occupied: Vec<TilePos> = self
            .monsters
            .iter()
            .map(|monster| (monster.x, monster.y))
            .chain(std::iter::once((self.player_x, self.player_y)))
            .collect();

        for i in 0..self.monsters.len() {
            self.monsters[i].cooldown -= dt;

            if self.monsters[i].cooldown <= 0.0 {
                self.monsters[i].cooldown = 1.0;

                let monster_position = (self.monsters[i].x, self.monsters[i].y);
                let player_position = (self.player_x, self.player_y);

                let distance = dist(monster_position, player_position);

                if distance == 1 {
                    self.hp -= 5;

                    let (screen_x, screen_y) = to_screen(self.player_x, self.player_y, self.camera);

                    self.texts.push(DmgText {
                        x: screen_x,
                        y: screen_y,
                        dmg: 5,
                        life: 1.0,
                        color: RED,
                    });
                } else {
                    let path = bfs(&self.map, monster_position, player_position);

                    if path.len() > 1 && !occupied.contains(&path[0]) {
                        self.monsters[i].x = path[0].0;
                        self.monsters[i].y = path[0].1;
                    }
                }
            }
        }

        false
    }

    fn collect_potion(&mut self) {
        let player_position = (self.player_x, self.player_y);

        if let Some(potion_index) = self
            .potions
            .iter()
            .position(|&potion| potion == player_position)
        {
            self.potions.remove(potion_index);
            self.hp += 25;

            if self.hp > 150 {
                self.hp = 150;
            }
        }
    }

    pub fn damage_monster(&mut self, idx: usize, amount: i32) {
        self.monsters[idx].hp -= amount;

        let (screen_x, screen_y) =
            to_screen(self.monsters[idx].x, self.monsters[idx].y, self.camera);

        self.texts.push(DmgText {
            x: screen_x,
            y: screen_y - 40.0,
            dmg: amount,
            life: 1.0,
            color: RED,
        });

        if self.monsters[idx].hp <= 0 {
            self.monsters.remove(idx);
            self.score += 50;
        }
    }

    pub fn draw(&self) {
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

        for (path_x, path_y) in &self.path {
            let (screen_x, screen_y) = to_screen(*path_x, *path_y, self.camera);
            draw_circle(screen_x, screen_y + 16.0, 4.0, GOLD);
        }

        draw_stickman(self.player_x, self.player_y, self.camera, false);

        for monster in &self.monsters {
            draw_stickman(monster.x, monster.y, self.camera, true);
        }

        for text in &self.texts {
            if text.dmg < 0 {
                draw_text(&format!("+{}", -text.dmg), text.x, text.y, 20.0, text.color);
            } else {
                draw_text(&format!("-{}", text.dmg), text.x, text.y, 20.0, text.color);
            }
        }

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
}
