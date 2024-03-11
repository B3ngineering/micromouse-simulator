mod mouse;
mod maze;
mod astar;
mod flood;
use std::ffi::c_double;
use std::vec;
use std::sync::Arc;
use druid::kurbo::{Point, Size};
use druid::widget::prelude::*;
use druid::{widget::{Button, Flex, Label, Painter}, AppLauncher, Color, Data, Lens, Widget, WidgetExt, WindowDesc};
use flood::flood_fill;
use std::thread;
use std::time::Duration;

#[derive(Clone, Data, Lens)]

struct CompData {
    mouse: mouse::Mouse,
    maze: maze::Maze,
    path: Arc<Vec<(i32, i32)>>,
    distances: Arc<Vec<Vec<i32>>>,
    explore_cost: i32,
    travel_cost: c_double
}

fn traverse_path(data: &mut CompData) {
   for node in (*data.path).clone() {
       data.mouse.x = node.0;
       data.mouse.y = -node.1;
       thread::sleep(Duration::from_millis(50));
   }
}

fn calculate_travel_cost(data: &mut CompData) {
    let mut node_prev = data.path[0];
    let mut iter = 0;
    let mut add_cost: c_double = 1.0;
    let mut direction_changed = false;
    let mut dir;

    if data.path[0].0 == data.path[1].0 {
        dir = "y";
    } else {
        dir = "x";
    }
    
    for node in (*data.path).clone() {
        if iter > 0 {
            node_prev = data.path[iter-1];
        }
        iter += 1;

        if iter == 1 {
            data.travel_cost += add_cost;
        } else if direction_changed {
            add_cost = 1.0;
            data.travel_cost += add_cost;
        } else {
            if add_cost >= 0.2 {
                add_cost = add_cost -  0.2;
            } else {
                add_cost = 0.0;
            }
            data.travel_cost += add_cost;
        }

        if dir == "x" {
            if node.0 == node_prev.0 && iter > 1 {
                direction_changed = true;
                dir = "y";
            } else {
                direction_changed = false;
                dir = "x";
            }
        } else {
            if node.1 == node_prev.1 && iter > 1 {
                direction_changed = true;
                dir = "x";
            } else {
                direction_changed = false;
                dir = "y";
            }
        }
    }
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
                } else if data.path.contains(&(i.try_into().unwrap(), j.try_into().unwrap())) {
                    ctx.fill(rect, &Color::BLUE); // Highlight color for specific cells
                    
                }
            }
        }
    }).fix_size(200.0, 200.0); // Fixed size for simplicity

    let mouse_position_label = Label::dynamic(|data: &CompData, _env| {
        format!("Mouse Position: ({}, {})", data.mouse.x, -data.mouse.y)
    });

    let north_button = Button::new("North")
        .on_click(|_ctx, data: &mut CompData, _env| {data.mouse.move_north()});
    let south_button = Button::new("South")
        .on_click(|_ctx, data: &mut CompData, _env| {data.mouse.move_south()});
    let east_button = Button::new("East")
        .on_click(|_ctx, data: &mut CompData, _env| data.mouse.move_east());
    let west_button = Button::new("West")
        .on_click(|_ctx, data: &mut CompData, _env| data.mouse.move_west());
    let astar_button = Button::new("A-Star")
       .on_click(|_ctx, data: &mut CompData, _env| {
            data.path = astar::astar((1, 1), (13, 13), (*data.maze.grid).clone()).into();
        });
    let flood_fill_button = Button::new("Flood Fill")
        .on_click(|_ctx, data: &mut CompData, _env| {
            data.path = Arc::new(flood_fill(&mut data.mouse, &data.maze, 13, 13));
        });
    let clear_button = Button::new("Reset")
       .on_click(|_ctx, data: &mut CompData, _env| {
        data.path = Arc::new(Vec::new());
        data.mouse.x = 1;
        data.mouse.y = -1;
        data.travel_cost = 0.0;
    });
    let traverse_button = Button::new("Traverse")
       .on_click(|_ctx, data: &mut CompData, _env| {
            traverse_path(data);
            calculate_travel_cost(data);
        });

    let trave_cost_label = Label::new(|data: &CompData, _env: &_| format!("Travel Cost = {:.1}", data.travel_cost)).center();

    Flex::column()
    .with_child(maze_painter)
    .with_child(mouse_position_label)
    .with_spacer(8.0)
    .with_child(
        Flex::row()
            .with_flex_spacer(1.0)
            .with_child(north_button)
            .with_child(south_button)
            .with_child(east_button)
            .with_child(west_button)
            .with_child(astar_button)
            .with_child(flood_fill_button)
            .with_child(clear_button)
            .with_child(traverse_button)
            .with_flex_spacer(1.0),
    )
    .with_child(trave_cost_label)

}

fn main() {
    let maze_grid = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
        vec![1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1],
        vec![1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1],
        vec![1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
        vec![1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];
    

    let arc_maze_grid = Arc::new(maze_grid);

    let main_window = WindowDesc::new(ui_builder())
        .title("Micromouse");

    let initial_state = CompData {
        mouse: mouse::Mouse::new(1, -1), // Starting position of the mouse
        maze: maze::Maze::new(arc_maze_grid),
        path: Arc::new(vec![(1,1)]),
        distances: Arc::new(vec![vec![]]),
        travel_cost: 0.0,
        explore_cost: 0
    };
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");

}
