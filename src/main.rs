const ROWS: usize = 3;
const COLS: usize = 3;

#[derive(Copy, Clone)]
enum State {
    Empty,
    X,
    O
}

impl State {
    fn Display() -> String {
        match {
            Empty: ' '.to_string(),
            X: 'X'.to_string(),
            O: 'O'.to_string(),
        }
    }
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
            state: State::X,
            x: x,
            y: y
        }
    }
}

fn show_grid(grid: Vec<Cell>) {
    for (i, cell) in grid.iter().enumerate() {
        match i % 3 {
            0 => println!("first column"),
            1 => println!("second column"),
            2 => println!("third column"),
            _ => panic!("not a valid column value"),
        }
        match cell.state {
            Empty => println!("Empty"),
            X => println!("X"),
            O => println!("O"),
            //_ => panic!("Not a valid state"),
        }
    }
}

fn is_gameover(grid: Vec<Cell>) -> String {
    return '1'.to_string();
}

fn play(x: u8, y: u8, state: State) {
    println!("{} is being played on ({}, {})", state, x, y);
}

fn main () {
    let mut grid: Vec<Cell> = vec![];

    for y in 0..ROWS {
        for x in 0..COLS {
            grid.push(Cell::new(x as u8, y as u8));
        }
    }
    play(2, 2, State::X);
    show_grid(grid.clone());
    is_gameover(grid.clone());
}

