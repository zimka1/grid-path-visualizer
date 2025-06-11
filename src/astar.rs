// Implements the step-by-step A* pathfinding algorithm with visualization

use crate::grid::draw_grid;
use crate::types::{Cell, CellType, State};
use pixels::Pixels;
use std::{cmp::Reverse, collections::BinaryHeap, collections::HashMap, thread, time::Duration};

// Manhattan distance used as heuristic
fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    let dx = (a.0 as isize - b.0 as isize).abs() as usize;
    let dy = (a.1 as isize - b.1 as isize).abs() as usize;
    dx + dy
}

// Executes A* and visualizes the process live
pub fn a_star_step_by_step(
    field: &mut Vec<Vec<Cell>>,
    start: (usize, usize),
    goal: (usize, usize),
    pixels: &mut Pixels,
    height: usize,
    width: usize,
) -> bool {
    let mut queue = BinaryHeap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut g_score = vec![vec![usize::MAX; width]; height];
    g_score[start.0][start.1] = 0;

    queue.push(Reverse(State {
        position: start,
        priority: heuristic(start, goal),
    }));

    let mut found = false;

    // Main loop
    while let Some(Reverse(State { position, .. })) = queue.pop() {
        if position == goal {
            found = true;
            break;
        }

        let (y, x) = position;

        if field[y][x].cell_type != CellType::Start && field[y][x].cell_type != CellType::Goal {
            field[y][x].cell_type = CellType::Visited;
        }

        // Redraw with updated cell
        draw_grid(pixels.frame_mut(), field, height, width);
        pixels.render().unwrap();
        thread::sleep(Duration::from_millis(100));

        // Explore 4 directions
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

    // If no path found, report and exit
    if !found {
        println!("No path found.");
        return false;
    }

    // Reconstruct the path from goal to start
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

    true
}
