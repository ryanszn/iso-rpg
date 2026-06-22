use macroquad::prelude::*;

pub type TilePos = (usize, usize);
pub type ScreenPos = (f32, f32);

pub enum AppState {
    Menu,
    Playing,
    GameOver,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
}

pub struct Monster {
    pub x: usize,
    pub y: usize,
    pub hp: i32,
    pub cooldown: f32,
}

pub struct DmgText {
    pub x: f32,
    pub y: f32,
    pub dmg: i32,
    pub life: f32,
    pub color: Color,
}
