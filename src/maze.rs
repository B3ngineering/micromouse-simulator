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

    pub fn is_wall(&self, x: i32, y:i32) -> bool {
        if self.grid[y as usize][x as usize] == 1 { return true}
        else {return false};
    }

    pub fn print(&self) {
        let grid = &self.grid;
        for row in (*grid).iter() {
            for cell in row {
                print!("{}", if *cell == 1 { "#" } else { " "});
            }
            println!();
        }
    }
}

