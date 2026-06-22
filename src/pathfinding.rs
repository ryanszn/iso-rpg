use std::collections::VecDeque;

use crate::constants::MAP;
use crate::types::Tile;

pub fn bfs(
    map: &[[Tile; MAP]; MAP],
    start: (usize, usize),
    goal: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut q = VecDeque::from([start]);
    let mut visited = [[false; MAP]; MAP];
    visited[start.1][start.0] = true;

    let mut parent = [[None; MAP]; MAP];

    while let Some(curr) = q.pop_front() {
        if curr == goal {
            let mut path = vec![];
            let mut c = goal;

            while c != start {
                path.push(c);
                c = parent[c.1][c.0].unwrap();
            }

            path.reverse();
            return path;
        }

        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let (nx, ny) = ((curr.0 as i32 + dx) as usize, (curr.1 as i32 + dy) as usize);

            if nx < MAP && ny < MAP && !visited[ny][nx] && map[ny][nx] == Tile::Floor {
                visited[ny][nx] = true;
                parent[ny][nx] = Some(curr);
                q.push_back((nx, ny));
            }
        }
    }

    vec![]
}
