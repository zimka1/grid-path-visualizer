// Cargo.toml dependencies:
// [dependencies]
// pixels = "0.13"
// winit = "0.29"
// log = "0.4"
// env_logger = "0.11"

use pixels::{Pixels, SurfaceTexture};
use std::time::Duration;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const CELL_SIZE: u32 = 20;
const COLORS: [(u8, u8, u8); 6] = [
    (255, 255, 255), // Empty: White
    (80, 80, 80),    // Wall: Dark Gray
    (0, 255, 0),     // Start: Green
    (255, 0, 0),     // Goal: Red
    (0, 0, 255),     // Visited: Blue
    (255, 255, 0),   // Path: Yellow
];

#[derive(Clone, Copy, PartialEq)]
enum CellType {
    Empty,
    Wall,
    Start,
    Goal,
    Visited,
    Path,
}

#[derive(Clone, Copy)]
struct Cell {
    cell_type: CellType,
}

fn draw_grid(frame: &mut [u8], field: &Vec<Vec<Cell>>, width: usize, height: usize) {
    for x in 0..width {
        for y in 0..height {
            let color = match field[x][y].cell_type {
                CellType::Empty => COLORS[0],
                CellType::Wall => COLORS[1],
                CellType::Start => COLORS[2],
                CellType::Goal => COLORS[3],
                CellType::Visited => COLORS[4],
                CellType::Path => COLORS[5],
            };
            fill_rect(
                frame,
                x as u32 * CELL_SIZE,
                y as u32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                color,
                width as u32 * CELL_SIZE,
            );
        }
    }
}

fn fill_rect(
    frame: &mut [u8],
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    (r, g, b): (u8, u8, u8),
    screen_width: u32,
) {
    for dx in 0..w {
        for dy in 0..h {
            let i = ((y + dy) * screen_width + (x + dx)) as usize * 4;
            frame[i] = r;
            frame[i + 1] = g;
            frame[i + 2] = b;
            frame[i + 3] = 255;
        }
    }
}

fn main() {
    let width = 10;
    let height = 10;
    let mut field = vec![
        vec![Cell { cell_type: CellType::Empty }; height];
        width
    ];

    field[2][0].cell_type = CellType::Start;
    field[width - 1][height - 1].cell_type = CellType::Goal;
    field[2][2].cell_type = CellType::Wall;
    field[2][3].cell_type = CellType::Wall;
    field[1][2].cell_type = CellType::Wall;

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

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(std::time::Instant::now() + Duration::from_millis(100));

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                draw_grid(pixels.get_frame(), &field, width, height);
                pixels.render().unwrap();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
