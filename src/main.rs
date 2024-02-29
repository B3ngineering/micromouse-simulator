mod mouse;
mod maze;
use std::sync::Arc;

use druid::kurbo::{Point, Size};
use druid::widget::prelude::*;
use druid::{widget::{Button, Flex, Painter}, AppLauncher, Color, Data, Lens, Widget, WidgetExt, WindowDesc};

#[derive(Clone, Data, Lens)]

struct CompData {
    mouse: mouse::Mouse,
    maze: maze::Maze,
}

fn ui_builder() -> impl Widget<CompData> {
    let maze_painter = Painter::new(|ctx, data: &CompData, _env| {
        let size = ctx.size();
        let cell_size = size.width / data.maze.grid[0].len() as f64;

        for (i, row) in data.maze.grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                let rect = Size::new(cell_size, cell_size)
                    .to_rect()
                    .with_origin(Point::new(i as f64 * cell_size, j as f64 * cell_size));
                
                ctx.fill(rect, if cell == 1 { &Color::BLACK } else { &Color::WHITE });

                // Highlight the mouse position
                if i == data.mouse.x as usize && j == -data.mouse.y as usize {
                    ctx.fill(rect, &Color::RED);
                }
            }
        }
    }).fix_size(200.0, 200.0); // Fixed size for simplicity

    let north_button = Button::new("North")
        .on_click(|_ctx, data: &mut CompData, _env| data.mouse.move_north());
    let south_button = Button::new("South")
        .on_click(|_ctx, data: &mut CompData, _env| data.mouse.move_south());
    let east_button = Button::new("East")
        .on_click(|_ctx, data: &mut CompData, _env| data.mouse.move_east());
    let west_button = Button::new("West")
        .on_click(|_ctx, data: &mut CompData, _env| data.mouse.move_west());


    Flex::column()
    .with_child(maze_painter)
    .with_spacer(8.0)
    .with_child(
        Flex::row()
            .with_flex_spacer(1.0)
            .with_child(north_button)
            .with_child(south_button)
            .with_child(east_button)
            .with_child(west_button)
            .with_flex_spacer(1.0),
    )
}

fn main() {
    println!("Hello, world!");
    let maze_grid = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
        vec![1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1],
        vec![1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
        vec![1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];
    let arc_maze_grid = Arc::new(maze_grid);
    //Window
    //Launch
    let main_window = WindowDesc::new(ui_builder())
        .title("Micromouse");

    let initial_state = CompData {
        mouse: mouse::Mouse::new(1, -1), // Starting position of the mouse
        maze: maze::Maze::new(arc_maze_grid),
    };
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");

}
