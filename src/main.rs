// Import necessary modules and dependencies
use pixels::{Pixels, SurfaceTexture};
use std::collections::{BinaryHeap, HashMap};
use std::time::{Duration};
use std::{thread, cmp::Reverse};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use std::cmp::Ordering;

// Size of each cell in pixels
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

// Enum for cell types
#[derive(Clone, Copy, PartialEq)]
enum CellType {
    Empty,
    Wall,
    Start,
    Goal,
    Visited,
    Path,
}

// Structure representing a grid cell
#[derive(Clone, Copy)]
struct Cell {
    cell_type: CellType,
}

// State struct used in the priority queue (BinaryHeap)
#[derive(Eq, PartialEq)]
struct State {
    position: (usize, usize),
    priority: usize,
}

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


// Manhattan distance heuristic for A* algorithm
fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    let dx = (a.0 as isize - b.0 as isize).abs() as usize;
    let dy = (a.1 as isize - b.1 as isize).abs() as usize;
    dx + dy
}

// Draw the entire grid to the screen (pixel frame buffer)
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

// Fill a rectangle in the pixel buffer with a solid color
fn draw_cell_with_border(
    frame: &mut [u8],
    x: u32,
    y: u32,
    size: u32,
    (r, g, b): (u8, u8, u8),
    screen_width: u32,
) {
    let inner_margin = 1;
    for dx in 0..size {
        for dy in 0..size {
            let i = ((y + dy) * screen_width + (x + dx)) as usize * 4;
            let is_border = dx == 0 || dx == size - 1 || dy == 0 || dy == size - 1;

            if is_border {
                frame[i] = 40;
                frame[i + 1] = 40;
                frame[i + 2] = 40;
                frame[i + 3] = 255;
            } else {
                frame[i] = r;
                frame[i + 1] = g;
                frame[i + 2] = b;
                frame[i + 3] = 255;
            }
        }
    }
}

// A* algorithm step-by-step with visualization after each step
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

    while let Some(Reverse(State { position, .. })) = queue.pop() {
        if position == goal {
            break;
        }

        let (y, x) = position;

        if field[y][x].cell_type != CellType::Start && field[y][x].cell_type != CellType::Goal {
            field[y][x].cell_type = CellType::Visited;
        }

        draw_grid(pixels.frame_mut(), field, height, width);
        pixels.render().unwrap();
        thread::sleep(Duration::from_millis(200));

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

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

    let mut current = goal;
    while current != start {
        current = came_from[&current];
        if current != start {
            field[current.0][current.1].cell_type = CellType::Path;

            draw_grid(pixels.frame_mut(), field, height, width);
            pixels.render().unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    }
}

// Entry point
fn main() {
    let height = 10;
    let width = 10;

    let mut field = vec![vec![Cell { cell_type: CellType::Empty }; width]; height];

    field[2][0].cell_type = CellType::Start;
    field[height - 1][width - 1].cell_type = CellType::Goal;
    field[2][2].cell_type = CellType::Wall;
    field[2][3].cell_type = CellType::Wall;
    field[1][2].cell_type = CellType::Wall;
    field[3][2].cell_type = CellType::Wall;

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

    let mut pixels = {
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
        Pixels::new(width as u32 * CELL_SIZE, height as u32 * CELL_SIZE, surface_texture).unwrap()
    };

    let mut should_run_astar = true;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            Event::MainEventsCleared => {
                if should_run_astar {
                    should_run_astar = false;
                    a_star_step_by_step(&mut field, (2, 0), (height - 1, width - 1), &mut pixels, height, width);
                }
                window.request_redraw();
            }

            Event::RedrawRequested(_) => {
                draw_grid(pixels.frame_mut(), &field, height, width);
                pixels.render().unwrap();
            }

            _ => {}
        }
    });
}
