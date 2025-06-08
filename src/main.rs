use std::{io};

#[derive(Clone, Copy, PartialEq)]
enum CellType {
    Empty,
    Wall,
    Start,
    Goal,
    Visited,
    Path
}

#[derive(Clone, Copy)]
struct Cell {
    cell_type: CellType,
    cost: usize
}

fn write_field(field: &Vec<Vec<Cell>>, height: usize, width: usize){
    for i in 0..height{
        for j in 0..width{
            match field[i][j].cell_type{
                CellType::Empty => print!("."),
                CellType::Visited => print!("+"),
                CellType::Path => print!("*"),
                CellType::Wall => print!("#"),
                CellType::Goal => print!("G"),
                CellType::Start => print!("S")              
            }
        }
        println!();
    }
}

fn add_wall(field: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
    field[x][y].cell_type = CellType::Wall;
}
fn add_start(field: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
    field[x][y].cell_type = CellType::Start;
}
fn add_goal(field: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
    field[x][y].cell_type = CellType::Goal;
}
fn add_path(field: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
    field[x][y].cell_type = CellType::Path;
}


fn main() {
    let mut input = String::new();
    let width;
    let height;
    io::stdin()
        .read_line(&mut input)
        .expect("Error to read");
    width = input.trim().parse().expect("Error to convert");

    input.clear();

    io::stdin()
        .read_line(&mut input)
        .expect("Error to read");
    height = input.trim().parse().expect("Error to convert");

    let mut field: Vec<Vec<Cell>> = vec![
        vec![Cell { cell_type: CellType::Empty, cost: usize::MAX }; width];
        height
    ];
    
    write_field(&field, height, width);

    

    add_start(&mut field, 0, 0);
    add_goal(&mut field, height - 1, width - 1);

    write_field(&field, height, width);




}
