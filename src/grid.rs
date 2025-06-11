// Responsible for drawing cells and the grid

use crate::types::{Cell, CellType};
use pixels::Pixels;

// Size of each cell in pixels
pub const CELL_SIZE: u32 = 40;

// RGB values for each cell type
pub const COLORS: [(u8, u8, u8); 6] = [
    (255, 255, 255), // Empty
    (80, 80, 80),    // Wall
    (0, 255, 0),     // Start
    (255, 0, 0),     // Goal
    (0, 0, 255),     // Visited
    (255, 255, 0),   // Path
];

// Renders the full grid to the screen
pub fn draw_grid(frame: &mut [u8], field: &Vec<Vec<Cell>>, height: usize, width: usize) {
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

// Draws one cell with a darker border for grid visualization
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
                // Draw border
                frame[i] = 40;
                frame[i + 1] = 40;
                frame[i + 2] = 40;
                frame[i + 3] = 255;
            } else {
                // Fill cell
                frame[i] = r;
                frame[i + 1] = g;
                frame[i + 2] = b;
                frame[i + 3] = 255;
            }
        }
    }
}
