
use crate::maze::Maze; // Adjust import paths based on your project structure
use crate::mouse::Mouse; // Adjust import paths based on your project structure
use std::collections::VecDeque;

// Assuming Maze and Cell structs are defined in a way that supports these operations

pub fn flood_fill(mouse: &mut Mouse, maze: &Maze, target_x: usize, target_y: usize) -> Vec<(i32, i32)> {
    let mut distances = vec![vec![i32::MAX; maze.grid[0].len()]; maze.grid.len()];
    let mut to_visit = VecDeque::new();
    distances[target_y][target_x] = 0;
    to_visit.push_back((target_x, target_y));

    while let Some((x, y)) = to_visit.pop_front() {
        let current_distance = distances[y][x];
        let neighbors = maze.get_neighbors(x, y); // Implement get_neighbors to provide neighbor cells without walls

        for (nx, ny) in neighbors {
            if distances[ny][nx] > current_distance + 1 {
                distances[ny][nx] = current_distance + 1;
                to_visit.push_back((nx, ny));
            }
        }
    }

    navigate_maze(mouse, maze, distances)
}

pub fn navigate_maze(mouse: &mut Mouse, maze: &Maze, distances: Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    let (mut x, mut y) = (mouse.x as usize, -mouse.y as usize);
    let mut path = Vec::new();
    path.push((1,1));

    while distances[x][y] != 0 {
        let neighbors = maze.get_neighbors(x, y);
        let mut min_distance = i32::MAX;
        let mut next_step = (x, y);

        for (nx, ny) in neighbors {
            if distances[ny][nx] < min_distance {
                min_distance = distances[ny][nx];
                next_step = (nx, ny);
            }
        }
        let path_step = (next_step.0 as i32, next_step.1 as i32);
        path.push(path_step);
        // mouse.move_to(next_step.0 as i32, next_step.1 as i32); // Implement move_to in the Mouse struct to update its position
        let (new_x, new_y) = next_step;
        x = new_x;
        y = new_y;
    }

    return path;
}