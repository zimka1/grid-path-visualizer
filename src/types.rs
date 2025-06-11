// Contains core data structures and types used across the app

use std::cmp::Ordering;

// Different types a cell can have
#[derive(Clone, Copy, PartialEq)]
pub enum CellType {
    Empty,
    Wall,
    Start,
    Goal,
    Visited,
    Path,
}

// Represents a single cell in the grid
#[derive(Clone, Copy)]
pub struct Cell {
    pub cell_type: CellType,
}

// Modes for placing different elements
#[derive(PartialEq)]
pub enum PlacementMode {
    Wall,
    Start,
    Goal,
}

// A* search state (node) with priority (f = g + h)
#[derive(Eq, PartialEq)]
pub struct State {
    pub position: (usize, usize),
    pub priority: usize,
}

// Ordering implementation for priority queue (min-heap using Reverse)
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
