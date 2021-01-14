const ROWS: usize = 3;
const COLS: usize = 3;

#[derive(Debug, Copy, Clone)]
enum State {
    Empty,
    X,
    O
}

#[derive(Copy, Clone)]
struct Cell {
    state: State,
    x: u8,
    y: u8
}

impl Cell {
    fn new(x: u8, y: u8) -> Cell {
        Cell {
            state: State::Empty,
            x: x,
            y: y
        }
    }
}

fn show_grid(grid: Vec<Cell>) {
    let line: String = "|{}|{}|{}|".to_string();
    for (i, cell) in grid.iter().enumerate() {
        match i % 3 {
            0 => println!("first column"),
            1 => println!("second column"),
            2 => println!("third column"),
            _ => panic!("not a valid column value"),
        }
        match cell.state {
            State::Empty => println!("Empty"),
            State::X => println!("X"),
            State::O => println!("O"),
        }
    }
}

fn is_gameover(grid: Vec<Cell>) -> String {
    return '1'.to_string();
}

fn play(x: u8, y: u8, state: State) {
    println!("{:?} is being played on ({}, {})", state, x, y);
    if x > 3 || y > 3{
        println!("Coordinate not valid");
    }
}

fn play_cell(cell: &Cell) {
    play(cell.x, cell.y, cell.state);
}

fn find(grid: Vec<Cell>, x: u8, y: u8) {
    for cell in grid.iter() {
        match cell.state {
            State::O => continue,
            State::X => continue,
            State::Empty => println!("Cell is Empty"),
        }
        if cell.x == x && cell.y == y {
            play_cell(cell);
        }
    }
}

fn main () {
    let mut grid: Vec<Cell> = vec![];

    for y in 0..ROWS {
        for x in 0..COLS {
            grid.push(Cell::new(x as u8, y as u8));
        }
    }
    let cell = Cell {x: 1, y: 1, state: State::O};
    find(grid.clone(), cell.x, cell.y);
    play(2, 2, State::X);
    show_grid(grid.clone());
    is_gameover(grid.clone());
}

