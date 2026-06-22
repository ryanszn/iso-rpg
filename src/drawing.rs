use macroquad::prelude::*;

use crate::math::to_screen;
use crate::types::ScreenPos;

pub fn draw_stickman(x: usize, y: usize, camera: ScreenPos, enemy: bool) {
    let (screen_x, mut screen_y) = to_screen(x, y, camera);
    screen_y += 16.0;

    draw_shadow(screen_x, screen_y);
    draw_head(screen_x, screen_y, enemy);
    draw_body(screen_x, screen_y);
}

fn draw_shadow(screen_x: f32, screen_y: f32) {
    draw_ellipse(
        screen_x,
        screen_y + 3.0,
        10.0,
        5.0,
        0.0,
        Color::new(0.0, 0.0, 0.0, 0.2),
    );
}

fn draw_head(screen_x: f32, screen_y: f32, enemy: bool) {
    draw_circle_lines(screen_x, screen_y - 32.0, 7.0, 2.0, BLACK);

    if enemy {
        draw_line(
            screen_x - 5.0,
            screen_y - 32.0,
            screen_x,
            screen_y - 30.0,
            2.0,
            BLACK,
        );

        draw_line(
            screen_x + 5.0,
            screen_y - 32.0,
            screen_x,
            screen_y - 30.0,
            2.0,
            BLACK,
        );
    }
}

fn draw_body(screen_x: f32, screen_y: f32) {
    let body_lines = [
        (0.0, -25.0, 0.0, -8.0),
        (0.0, -20.0, -8.0, -15.0),
        (0.0, -20.0, 8.0, -15.0),
        (0.0, -8.0, -6.0, 0.0),
        (0.0, -8.0, 6.0, 0.0),
    ];

    for (x1, y1, x2, y2) in body_lines {
        draw_line(
            screen_x + x1,
            screen_y + y1,
            screen_x + x2,
            screen_y + y2,
            2.0,
            BLACK,
        );
    }
}

pub fn draw_wall(x: usize, y: usize, camera: ScreenPos) {
    let (screen_x, screen_y) = to_screen(x, y, camera);

    let vertices = [
        vec2(screen_x, screen_y - 40.0),
        vec2(screen_x + 32.0, screen_y - 24.0),
        vec2(screen_x, screen_y - 8.0),
        vec2(screen_x - 32.0, screen_y - 24.0),
        vec2(screen_x + 32.0, screen_y),
        vec2(screen_x, screen_y + 16.0),
        vec2(screen_x - 32.0, screen_y),
    ];

    let face_colors = [
        Color::new(0.8, 0.8, 0.8, 1.0),
        Color::new(0.5, 0.5, 0.5, 1.0),
        Color::new(0.6, 0.6, 0.6, 1.0),
    ];

    draw_triangle(vertices[0], vertices[1], vertices[2], face_colors[0]);
    draw_triangle(vertices[0], vertices[2], vertices[3], face_colors[0]);

    draw_triangle(vertices[1], vertices[4], vertices[5], face_colors[1]);
    draw_triangle(vertices[1], vertices[5], vertices[2], face_colors[1]);

    draw_triangle(vertices[3], vertices[2], vertices[5], face_colors[2]);
    draw_triangle(vertices[3], vertices[5], vertices[6], face_colors[2]);

    for (a, b) in [(0, 1), (1, 2), (2, 3), (3, 0), (1, 4), (2, 5), (3, 6)] {
        draw_line(
            vertices[a].x,
            vertices[a].y,
            vertices[b].x,
            vertices[b].y,
            1.0,
            BLACK,
        );
    }
}
