use std::cmp::Reverse;
use std::{
    collections::{BinaryHeap, HashMap},
    io,
};

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
    cost: usize,
}

fn write_field(field: &Vec<Vec<Cell>>, height: usize, width: usize) {
    for i in 0..height {
        for j in 0..width {
            match field[i][j].cell_type {
                CellType::Empty => print!("."),
                CellType::Visited => print!("+"),
                CellType::Path => print!("*"),
                CellType::Wall => print!("#"),
                CellType::Goal => print!("G"),
                CellType::Start => print!("S"),
            }
        }
        println!();
    }
}

fn add_wall(field: &mut Vec<Vec<Cell>>, point: (usize, usize)) {
    field[point.0][point.1].cell_type = CellType::Wall;
}
fn add_start(field: &mut Vec<Vec<Cell>>, point: (usize, usize)) {
    field[point.0][point.1].cell_type = CellType::Start;
}
fn add_goal(field: &mut Vec<Vec<Cell>>, point: (usize, usize)) {
    field[point.0][point.1].cell_type = CellType::Goal;
}
fn add_path(field: &mut Vec<Vec<Cell>>, point: (usize, usize)) {
    field[point.0][point.1].cell_type = CellType::Path;
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct State {
    position: (usize, usize),
    priority: usize,
}

fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    let dx = (a.0 as isize - b.0 as isize).abs() as usize;
    let dy = (a.1 as isize - b.1 as isize).abs() as usize;
    dx + dy
}

fn a_star(
    field: &mut Vec<Vec<Cell>>,
    height: usize,
    width: usize,
    start: (usize, usize),
    goal: (usize, usize),
) {
    let mut queue = BinaryHeap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut g_score = vec![vec![usize::MAX; width]; height];

    g_score[start.0][start.1] = 0;

    queue.push(Reverse(State {
        position: start,
        priority: heuristic(start, goal),
    }));

    while let Some(Reverse(State { position, priority })) = queue.pop() {
        let (x, y) = position;

        if goal == position {
            break;
        }

        if field[x][y].cell_type != CellType::Start && field[x][y].cell_type != CellType::Goal {
            field[x][y].cell_type = CellType::Visited;
        }

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for (dx, dy) in directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0
                || ny < 0
                || ny >= width as isize
                || nx >= height as isize
                || field[nx as usize][ny as usize].cell_type == CellType::Wall
            {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if field[nx][ny].cell_type == CellType::Wall {
                continue;
            }
            let tentative_g_score = g_score[x][y] + 1;
            if tentative_g_score < g_score[nx][ny] {
                g_score[nx][ny] = tentative_g_score;
                came_from.insert((nx, ny), (x, y));
                let f_score = tentative_g_score + heuristic((nx, ny), goal);
                queue.push(Reverse(State {
                    position: (nx, ny),
                    priority: f_score,
                }));
            }
        }
    }

    let mut current = goal;
    while current != start {
        current = came_from[&current];
        if current != start {
            add_path(field, current);
        }
    }

    write_field(field, height, width);
}

fn main() {
    let mut input = String::new();
    let width;
    let height;
    io::stdin().read_line(&mut input).expect("Error to read");
    height = input.trim().parse().expect("Error to convert");

    input.clear();

    io::stdin().read_line(&mut input).expect("Error to read");
    width = input.trim().parse().expect("Error to convert");

    let mut field: Vec<Vec<Cell>> = vec![
        vec![
            Cell {
                cell_type: CellType::Empty,
                cost: usize::MAX
            };
            width
        ];
        height
    ];

    write_field(&field, height, width);

    println!("");

    let start: (usize, usize) = (2, 0);
    let goal: (usize, usize) = (height - 1, width - 1);

    add_start(&mut field, start);
    add_goal(&mut field, goal);

    add_wall(&mut field, (2, 2));
    add_wall(&mut field, (2, 3));
    add_wall(&mut field, (1, 2));

    a_star(&mut field, height, width, start, goal);
}
