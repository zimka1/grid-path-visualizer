// Import necessary modules and dependencies
use pixels::{Pixels, SurfaceTexture};
use std::collections::{BinaryHeap, HashMap};
use std::time::Duration;
use std::{cmp::Reverse, thread};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use std::cmp::Ordering;

// Size of each grid cell in pixels
const CELL_SIZE: u32 = 40;

// RGB colors for each cell type
const COLORS: [(u8, u8, u8); 6] = [
    (255, 255, 255), // Empty: White
    (80, 80, 80),    // Wall: Dark Gray
    (0, 255, 0),     // Start: Green
    (255, 0, 0),     // Goal: Red
    (0, 0, 255),     // Visited: Blue
    (255, 255, 0),   // Path: Yellow
];

// Types of cells used in the grid
#[derive(Clone, Copy, PartialEq)]
enum CellType {
    Empty,
    Wall,
    Start,
    Goal,
    Visited,
    Path,
}

// Representation of a cell in the grid
#[derive(Clone, Copy)]
struct Cell {
    cell_type: CellType,
}

// Modes for placing elements before the algorithm runs
#[derive(PartialEq)]
enum PlacementMode {
    Wall,
    Start,
    Goal,
}

// State used in the priority queue for A*
#[derive(Eq, PartialEq)]
struct State {
    position: (usize, usize),
    priority: usize,
}

// Priority ordering based on lowest cost first (min-heap)
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Manhattan distance heuristic for A*
fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    let dx = (a.0 as isize - b.0 as isize).abs() as usize;
    let dy = (a.1 as isize - b.1 as isize).abs() as usize;
    dx + dy
}

// Draw the current state of the grid to the pixel buffer
fn draw_grid(frame: &mut [u8], field: &Vec<Vec<Cell>>, height: usize, width: usize) {
    for y in 0..height {
        for x in 0..width {
            let color = match field[y][x].cell_type {
                CellType::Empty => COLORS[0],
                CellType::Wall => COLORS[1],
                CellType::Start => COLORS[2],
                CellType::Goal => COLORS[3],
                CellType::Visited => COLORS[4],
                CellType::Path => COLORS[5],
            };
            draw_cell_with_border(
                frame,
                x as u32 * CELL_SIZE,
                y as u32 * CELL_SIZE,
                CELL_SIZE,
                color,
                width as u32 * CELL_SIZE,
            );
        }
    }
}

// Draw a colored cell with a border at given screen position
fn draw_cell_with_border(
    frame: &mut [u8],
    x: u32,
    y: u32,
    size: u32,
    (r, g, b): (u8, u8, u8),
    screen_width: u32,
) {
    for dx in 0..size {
        for dy in 0..size {
            let i = ((y + dy) * screen_width + (x + dx)) as usize * 4;
            let is_border = dx == 0 || dx == size - 1 || dy == 0 || dy == size - 1;

            if is_border {
                // Draw cell border
                frame[i] = 40;
                frame[i + 1] = 40;
                frame[i + 2] = 40;
                frame[i + 3] = 255;
            } else {
                // Draw inner cell color
                frame[i] = r;
                frame[i + 1] = g;
                frame[i + 2] = b;
                frame[i + 3] = 255;
            }
        }
    }
}

// A* pathfinding algorithm with visual step-by-step updates
fn a_star_step_by_step(
    field: &mut Vec<Vec<Cell>>,
    start: (usize, usize),
    goal: (usize, usize),
    pixels: &mut Pixels,
    height: usize,
    width: usize,
) {
    let mut queue = BinaryHeap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut g_score = vec![vec![usize::MAX; width]; height];
    g_score[start.0][start.1] = 0;

    queue.push(Reverse(State {
        position: start,
        priority: heuristic(start, goal),
    }));

    // Main A* loop
    while let Some(Reverse(State { position, .. })) = queue.pop() {
        if position == goal {
            break;
        }

        let (y, x) = position;

        // Mark the cell as visited
        if field[y][x].cell_type != CellType::Start && field[y][x].cell_type != CellType::Goal {
            field[y][x].cell_type = CellType::Visited;
        }

        // Draw updated frame
        draw_grid(pixels.frame_mut(), field, height, width);
        pixels.render().unwrap();
        thread::sleep(Duration::from_millis(100));

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        // Explore neighbors
        for (dy, dx) in directions {
            let ny = y as isize + dy;
            let nx = x as isize + dx;

            if ny < 0 || nx < 0 || ny >= height as isize || nx >= width as isize {
                continue;
            }

            let (ny, nx) = (ny as usize, nx as usize);

            if field[ny][nx].cell_type == CellType::Wall {
                continue;
            }

            let tentative_g = g_score[y][x] + 1;

            if tentative_g < g_score[ny][nx] {
                g_score[ny][nx] = tentative_g;
                came_from.insert((ny, nx), (y, x));
                let f_score = tentative_g + heuristic((ny, nx), goal);
                queue.push(Reverse(State {
                    position: (ny, nx),
                    priority: f_score,
                }));
            }
        }
    }

    // Trace and draw final path
    let mut current = goal;
    while current != start {
        current = came_from[&current];
        if current != start {
            field[current.0][current.1].cell_type = CellType::Path;
            draw_grid(pixels.frame_mut(), field, height, width);
            pixels.render().unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    }
}

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
