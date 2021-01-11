const rows: usize = 3;
const cols: usize = 3;


enum State {
    Empty,
    X,
    O
}

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
    for (i, cell) in grid.iter().enumerate() {
        match i % 3 {
            0 => println!("first column"),
            1 => println!("second column"),
            2 => println!("third column"),
            _ => panic!("not a valid column value"),
        }
        match cell.state {
            Empty => println!(" "),
            X => println!("X"),
            O => println!("O"),
            _ => panic!("Not a valid state"),
        }
    }
}

fn is_gameover(grid: [[Cell; cols]; rows]) -> String {
    return '1'.to_string();
}

fn main () {
    let mut cells: Vec<Cell> = vec![];

    for y in 0..rows {
        for x in 0..cols {
            cells.push(Cell::new(x as u8, y as u8));
        }
    }
    show_grid(cells);
}

