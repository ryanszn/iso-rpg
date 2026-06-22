use crate::constants::TILE_SIZE;
use crate::types::{ScreenPos, TilePos};

pub fn to_screen(x: usize, y: usize, camera: ScreenPos) -> ScreenPos {
    let tile_x = x as f32;
    let tile_y = y as f32;

    let screen_x = (tile_x - tile_y) * TILE_SIZE.0 + camera.0;
    let screen_y = (tile_x + tile_y) * TILE_SIZE.1 + camera.1;

    (screen_x, screen_y)
}

pub fn to_tile(screen_x: f32, screen_y: f32, camera: ScreenPos) -> TilePos {
    let adjusted_x = screen_x - camera.0;
    let adjusted_y = screen_y - camera.1;

    let iso_x = adjusted_x / TILE_SIZE.0;
    let iso_y = adjusted_y / TILE_SIZE.1;

    let tile_x = ((iso_x + iso_y) / 2.0) as usize;
    let tile_y = ((iso_y - iso_x) / 2.0) as usize;

    (tile_x, tile_y)
}

pub fn dist(p1: TilePos, p2: TilePos) -> i32 {
    let x_distance = (p1.0 as i32 - p2.0 as i32).abs();
    let y_distance = (p1.1 as i32 - p2.1 as i32).abs();

    x_distance + y_distance
}
