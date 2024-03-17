use druid::{Data, Lens};
use std::sync::Arc;
#[derive(Clone, Data, Lens)]
pub struct Maze {
    pub grid: Arc<Vec<Vec<i32>>>,
}

impl Maze {
    pub fn new(grid:Arc<Vec<Vec<i32>>>) -> Self {
        Maze { grid }
    }

    // Check if cell is a wall cell or open space
    pub fn is_wall(&self, x: i32, y:i32) -> bool {
        if self.grid[y as usize][x as usize] == 1 { return true}
        else {return false};
    }

    // Helper printing function
    pub fn print(&self) {
        let grid = &self.grid;
        for row in (*grid).iter() {
            for cell in row {
                print!("{}", if *cell == 1 { "#" } else { " "});
            }
            println!();
        }
    }

    // Get surroudning cells from current location
    pub fn get_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let rows = self.grid.len();
        let cols = self.grid[0].len();
    
        // Check above
        if row > 0 && self.grid[row - 1][col] == 0 {
            neighbors.push((row - 1, col));
        }
    
        // Check below
        if row + 1 < rows && self.grid[row + 1][col] == 0 {
            neighbors.push((row + 1, col));
        }
    
        // Check left
        if col > 0 && self.grid[row][col - 1] == 0 {
            neighbors.push((row, col - 1));
        }
    
        // Check right
        if col + 1 < cols && self.grid[row][col + 1] == 0 {
            neighbors.push((row, col + 1));
        }
    
        neighbors
    }
}

