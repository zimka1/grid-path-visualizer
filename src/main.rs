mod types;
mod grid;
mod astar;

use types::*;
use grid::*;
use astar::*;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

// Main function
fn main() {
    let height = 10;
    let width = 10;

    // Initialize the grid with empty cells
    let mut field = vec![vec![Cell { cell_type: CellType::Empty }; width]; height];

    // Create a window and event loop
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new((width as u32 * CELL_SIZE) as f64, (height as u32 * CELL_SIZE) as f64);
        WindowBuilder::new()
            .with_title("A* Grid Visualization")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    // Create pixel buffer for rendering
    let mut pixels = {
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
        Pixels::new(width as u32 * CELL_SIZE, height as u32 * CELL_SIZE, surface_texture).unwrap()
    };

    // Application state variables
    let mut path_drawn = false;
    let mut should_run_astar = false;
    let mut placing_walls = true;
    let mut cursor_position: Option<(f64, f64)> = None;
    let mut placement_mode = PlacementMode::Wall;

    // Store current start and goal positions
    let mut start_pos: Option<(usize, usize)> = None;
    let mut goal_pos: Option<(usize, usize)> = None;

    // Start the event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            // Handle window and input events
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                // Handle mouse clicks while placing elements
                WindowEvent::MouseInput { state, .. } if placing_walls => {
                    if state == winit::event::ElementState::Pressed {
                        if let Some(pos) = cursor_position {
                            let x = (pos.0 / CELL_SIZE as f64) as usize;
                            let y = (pos.1 / CELL_SIZE as f64) as usize;

                            if x < width && y < height {
                                match placement_mode {
                                    PlacementMode::Wall => {
                                        if field[y][x].cell_type == CellType::Wall {
                                            field[y][x].cell_type = CellType::Empty;
                                        } else if field[y][x].cell_type == CellType::Empty {
                                            field[y][x].cell_type = CellType::Wall;
                                        }
                                    }
                                    PlacementMode::Start => {
                                        if field[y][x].cell_type == CellType::Wall {
                                            return;
                                        }
                                        // Remove previous start
                                        for row in &mut field {
                                            for cell in row {
                                                if cell.cell_type == CellType::Start {
                                                    cell.cell_type = CellType::Empty;
                                                }
                                            }
                                        }
                                        if field[y][x].cell_type == CellType::Empty {
                                            field[y][x].cell_type = CellType::Start;
                                            start_pos = Some((y, x));
                                        }
                                    }
                                    PlacementMode::Goal => {
                                        if field[y][x].cell_type == CellType::Wall {
                                            return;
                                        }
                                        // Remove previous goal
                                        for row in &mut field {
                                            for cell in row {
                                                if cell.cell_type == CellType::Goal {
                                                    cell.cell_type = CellType::Empty;
                                                }
                                            }
                                        }
                                        if field[y][x].cell_type == CellType::Empty {
                                            field[y][x].cell_type = CellType::Goal;
                                            goal_pos = Some((y, x));
                                        }
                                    }
                                }
                                window.request_redraw();
                            }
                        }
                    }
                }

                // Track and convert mouse cursor position to logical coordinates
                WindowEvent::CursorMoved { position, .. } => {
                    let logical = position.to_logical::<f64>(window.scale_factor());
                    cursor_position = Some((logical.x, logical.y));
                }

                // Handle key presses to switch placement mode or run the algorithm
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        if input.state == winit::event::ElementState::Pressed {
                            match key {
                                VirtualKeyCode::W => placement_mode = PlacementMode::Wall,
                                VirtualKeyCode::S => placement_mode = PlacementMode::Start,
                                VirtualKeyCode::G => placement_mode = PlacementMode::Goal,
                                VirtualKeyCode::R => {
                                    if path_drawn {
                                        // Reset all visited and path cells
                                        for row in &mut field {
                                            for cell in row {
                                                if cell.cell_type == CellType::Path || cell.cell_type == CellType::Visited {
                                                    cell.cell_type = CellType::Empty;
                                                }
                                            }
                                        }
                                
                                        // Reset state
                                        placing_walls = true;
                                        should_run_astar = false;
                                        path_drawn = false;
                                        window.request_redraw();
                                        println!("Grid reset. Place walls, start, and goal again.");
                                    }
                                }
                                VirtualKeyCode::Space => {
                                    if placing_walls {
                                        if let (Some(start), Some(goal)) = (start_pos, goal_pos) {
                                            placing_walls = false;
                                            should_run_astar = true;
                                        } else {
                                            println!("Please set both start and goal positions before running.");
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }

                _ => {}
            },

            // Run A* algorithm after setup
            Event::MainEventsCleared => {
                if should_run_astar {
                    should_run_astar = false;
                    a_star_step_by_step(
                        &mut field,
                        start_pos.unwrap(),
                        goal_pos.unwrap(),
                        &mut pixels,
                        height,
                        width,
                    );
                    path_drawn = true;
                }                
                window.request_redraw();
            }

            // Redraw screen
            Event::RedrawRequested(_) => {
                draw_grid(pixels.frame_mut(), &field, height, width);
                pixels.render().unwrap();
            }

            _ => {}
        }
    });
}
