use std::collections::VecDeque;

use crate::constants::MAP_SIZE;
use crate::types::{Tile, TilePos};

pub fn bfs(map: &[[Tile; MAP_SIZE]; MAP_SIZE], start: TilePos, goal: TilePos) -> Vec<TilePos> {
    let mut queue = VecDeque::from([start]);

    let mut visited = [[false; MAP_SIZE]; MAP_SIZE];
    visited[start.1][start.0] = true;

    let mut parent = [[None; MAP_SIZE]; MAP_SIZE];

    while let Some(current) = queue.pop_front() {
        if current == goal {
            let mut path = vec![];
            let mut current_step = goal;

            while current_step != start {
                path.push(current_step);
                current_step = parent[current_step.1][current_step.0].unwrap();
            }

            path.reverse();
            return path;
        }

        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let next_x = current.0 as i32 + dx;
            let next_y = current.1 as i32 + dy;

            if next_x < 0 || next_y < 0 {
                continue;
            }

            let next_x = next_x as usize;
            let next_y = next_y as usize;

            if next_x < MAP_SIZE
                && next_y < MAP_SIZE
                && !visited[next_y][next_x]
                && map[next_y][next_x] == Tile::Floor
            {
                visited[next_y][next_x] = true;
                parent[next_y][next_x] = Some(current);
                queue.push_back((next_x, next_y));
            }
        }
    }

    vec![]
}
