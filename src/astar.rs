use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
// heuristic function

// get neighbours function

// Astar function

// updates ui

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Node {
    position: (i32, i32), // Using a tuple (x, y) to represent a position on the grid
    cost: i32, // `g` cost: cost from the start node to this node
    heuristic: i32, // `h` cost: estimated cost from this node to the goal
}

impl Node {
    fn total_cost(&self) -> i32 {
        self.cost + self.heuristic
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.total_cost().cmp(&self.total_cost())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(node: &(i32, i32), goal: &(i32, i32)) -> i32 {
    (node.0 - goal.0).abs() + (node.1 - goal.1).abs()
}

fn is_valid_position(pos: &(i32, i32), maze: &Vec<Vec<i32>>) -> bool {
    pos.0 >= 0 && pos.0 < maze[0].len() as i32 && pos.1 >= 0 && pos.1 < maze.len() as i32 && maze[pos.0 as usize][pos.1 as usize] == 0
}

fn get_neighbors(node: &(i32, i32), maze: &Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    let neighbors = vec![
        (node.0 - 1, node.1), // Left
        (node.0 + 1, node.1), // Right
        (node.0, node.1 - 1), // Up
        (node.0, node.1 + 1), // Down
        // Add diagonal neighbors if necessary
    ];
    neighbors.into_iter().filter(|pos| is_valid_position(pos, maze)).collect()
}

pub fn astar(start: (i32, i32), goal: (i32, i32), maze: Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut g_score: HashMap<(i32, i32), i32> = HashMap::new();

    // Initialize starting node
    open_set.push(Node {
        position: start,
        cost: 0,
        heuristic: heuristic(&start, &goal),
    });
    g_score.insert(start, 0);
    print!("Running Astar");
    while let Some(current) = open_set.pop() {
        print!("({}, {})", current.position.0, current.position.1);
        if current.position == goal {
            // Reconstruct path
            let mut path = Vec::new();
            let mut current_pos = goal;
            while current_pos != start {
                path.push(current_pos);
                current_pos = came_from[&current_pos];
            }
            path.push(start);
            path.reverse();
            for node in &path { print!("Official path: ({}, {})", node.0, node.1)}
            return path;
        }

        for neighbor in get_neighbors(&current.position, &maze) {
            let tentative_g_score = current.cost + 1; // Assuming uniform cost for simplicity

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                // This path to neighbor is better than any previous one
                came_from.insert(neighbor, current.position);
                g_score.insert(neighbor, tentative_g_score);
                open_set.push(Node {
                    position: neighbor,
                    cost: tentative_g_score,
                    heuristic: heuristic(&neighbor, &goal),
                });
            }

            
        }
    }

    return vec![] // No path was found
}