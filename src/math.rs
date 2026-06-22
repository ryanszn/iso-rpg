use crate::constants::T_SIZE;

pub fn to_screen(x: usize, y: usize, cam: (f32, f32)) -> (f32, f32) {
    (
        (x as f32 - y as f32) * T_SIZE.0 + cam.0,
        (x as f32 + y as f32) * T_SIZE.1 + cam.1,
    )
}

pub fn to_tile(sx: f32, sy: f32, cam: (f32, f32)) -> (usize, usize) {
    let (ax, ay) = (sx - cam.0, sy - cam.1);

    (
        ((ax / T_SIZE.0 + ay / T_SIZE.1) / 2.) as usize,
        ((ay / T_SIZE.1 - ax / T_SIZE.0) / 2.) as usize,
    )
}

pub fn dist(p1: (usize, usize), p2: (usize, usize)) -> i32 {
    (p1.0 as i32 - p2.0 as i32).abs() + (p1.1 as i32 - p2.1 as i32).abs()
}
