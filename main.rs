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
    fn init(x: u8, y: u8) -> &Cell {
        Cell {
            state: State::Empty,
            x: x,
            y: y
        }
    }
}

fn is_gameover(grid: [[Cell; cols]; rows]) -> String {
    return '1'.to_string();
}

fn main () {
    let mut grid: &[[Cell; cols]; rows];

    for y in 0..rows {
        for x in 0..cols {
            grid[y][x]: Cell = Cell.new(x, y);
        }
    }
    for (x, row) in grid.iter_mut().enumerate() {
        for (y, cell) in row.iter_mut().enumerate() {
            cell: Cell = Cell.new(x, y);
        }
    }
    let mut cells: Vec<Cell>;

    for y in 0..rows {
        for x in 0..cols {
            cells.push(Cell.new(x, y));
        }
    }
}

