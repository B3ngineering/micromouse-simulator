use druid::{Data, ExtEventSink, Lens};
#[derive(Clone, Copy, Data, Lens)]
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

        pub async fn travel_path (&mut self, event_sink: ExtEventSink, path: Vec<(i32, i32)>) {
//             for node in path {
//                 if node.0 > self.x { self.move_east()}
//                 else if node.0 < self.x {self.move_west()}
//                 else if node.1 > self.y {self.move_south()}
//                 else if node.1 < self.y {self.move_north()}

//                 sleep(Duration::from_millis(100)).await;
// sleep(Duration::from_millis(100)).await;
//                 if event_sink
//             .submit_command(UPDATE_UI, *self, druid::Target::Auto)
//             .is_err()
//         {
//             eprintln!("Failed to send command to update UI.");
//             break;
//         }
//             }
        }

    }