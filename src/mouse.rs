use druid::{Data, Lens};
#[derive(Clone, Data, Lens)]

    pub struct Mouse {
        pub x: i32,
        pub y: i32,
    }

    impl Mouse {

        //Constructor for a new mouse
        pub fn new(start_x: i32, start_y: i32) -> Self {
            Mouse { x: start_x, y: start_y }
        }

        pub fn move_north(&mut self) {
            self.y += 1;
            print!("({}, {})", self.x, self.y);
        }

        pub fn move_south(&mut self) {
            self.y -= 1;
            print!("({}, {})", self.x, self.y);
        }

        pub fn move_east(&mut self) {
            self.x += 1;
            print!("({}, {})", self.x, self.y);
        }

        pub fn move_west(&mut self) {
            self.x -= 1;
            print!("({}, {})", self.x, self.y);
        }

        pub fn move_by(&mut self, dx: i32, dy: i32) {
            self.x += dx;
            self.y -= dy;
        }

        pub fn get_position(&self) -> (i32, i32) {
            (self.x, self.y)
        }

        pub fn print_position(&self) {
            println!("Mouse Position: ({}, {})", self.x, self.y);
        }

    }