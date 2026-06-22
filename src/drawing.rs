use macroquad::prelude::*;

use crate::math::to_screen;

pub fn draw_stickman(x: usize, y: usize, cam: (f32, f32), enemy: bool) {
    let (sx, mut sy) = to_screen(x, y, cam);
    sy += 16.;

    // shadow
    draw_ellipse(sx, sy + 3., 10., 5., 0., Color::new(0., 0., 0., 0.2));

    // head
    if enemy {
        draw_line(sx - 5., sy - 32., sx, sy - 30., 2., BLACK);
        draw_line(sx + 5., sy - 32., sx, sy - 30., 2., BLACK);
    } else {
        draw_circle_lines(sx, sy - 32., 7., 2., BLACK);
    }

    draw_circle_lines(sx, sy - 32., 7., 2., BLACK);

    for l in [
        [0., -25., 0., -8.],
        [0., -20., -8., -15.],
        [0., -20., 8., -15.],
        [0., -8., -6., -0.],
        [0., -25., 6., -8.],
    ] {
        draw_line(sx + l[0], sy + l[1], sx + l[2], sy + l[3], 2., BLACK);
    }
}

pub fn draw_wall(x: usize, y: usize, cam: (f32, f32)) {
    let (sx, sy) = to_screen(x, y, cam);

    let v = [
        vec2(sx, sy - 40.),
        vec2(sx + 32., sy - 24.),
        vec2(sx, sy - 8.),
        vec2(sx - 32., sy - 24.),
        vec2(sx + 32., sy),
        vec2(sx, sy + 16.),
        vec2(sx - 32., sy),
    ];

    let colors = [
        Color::new(0.8, 0.8, 0.8, 1.),
        Color::new(0.5, 0.5, 0.5, 1.),
        Color::new(0.6, 0.6, 0.6, 1.),
    ];

    // Draw faces
    draw_triangle(v[0], v[1], v[2], colors[0]);
    draw_triangle(v[0], v[2], v[3], colors[0]);

    draw_triangle(v[1], v[4], v[5], colors[1]);
    draw_triangle(v[1], v[5], v[2], colors[1]);

    draw_triangle(v[3], v[2], v[5], colors[2]);
    draw_triangle(v[3], v[5], v[6], colors[2]);

    // Draw outline
    for (a, b) in [(0, 1), (1, 2), (2, 3), (3, 0), (1, 4), (2, 5), (3, 6)] {
        draw_line(v[a].x, v[a].y, v[b].x, v[b].y, 1., BLACK);
    }
}
