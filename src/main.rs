use std::fmt;
use std::io;
use std::io::Error as ioError;
use std::num::ParseIntError;

const ROWS: usize = 3;
const COLS: usize = 3;

// Custom error struct
#[derive(Debug)]
struct Error {
    description: String,
}

impl Error {
    fn new(err_description: String) -> Error {
        Error {
            description: err_description,
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::new(format!("ParseIntError: {}", e))
    }
}

impl From<ioError> for Error {
    fn from(e: ioError) -> Self {
        Error::new(format!("Input error: {}", e))
    }
}

// State struct
#[derive(Debug, Copy, Clone)]
enum State {
    Empty,
    X,
    O,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::Empty => write!(f, " "),
            State::X => write!(f, "X"),
            State::O => write!(f, "O"),
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (State::Empty, State::Empty) => true,
            (State::X, State::X) => true,
            (State::O, State::O) => true,
            (_, _) => false,
        }
    }
}

// Coordinates struct
#[derive(Copy, Clone, Debug)]
struct Coordinate {
    x: u8,
    y: u8,
}

impl Coordinate {
    fn new(x: u8, y: u8) -> Coordinate {
        Coordinate { x: x, y: y }
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

// Cell struct
#[derive(Copy, Clone, Debug)]
struct Cell {
    state: State,
    address: Coordinate,
}

impl Cell {
    fn new(x: u8, y: u8) -> Cell {
        Cell {
            state: State::Empty,
            address: Coordinate::new(x, y),
        }
    }
}

// Fuctions

fn show_grid(grid: &Vec<Cell>) {
    for (i, cell) in grid.iter().enumerate() {
        match (i + 1) % COLS {
            0 => println!("|{}|", cell.state),
            _ => print!("|{}", cell.state),
        }
    }
}

fn exist_winning_move(initial_cell: Cell, middle: &Cell, extreme: &Cell) -> bool {
    use State::*;
    match (initial_cell.state, middle.state, extreme.state) {
        (X, X, X) | (O, O, O) => {
            return true;
        }
        (_, _, _) => return false,
    };
}

fn is_gameover(grid: Vec<Cell>) -> bool {
    let grid_iterator = grid.iter();

    for cell in grid.clone() {
        let mut relative_cells_to_eval: Vec<(&Cell, &Cell)> = vec![];
        let middle_row = grid_iterator.clone().find(|&other_cell| {
            other_cell.address.x == cell.address.x + 1 && other_cell.address.y == cell.address.y
        });
        let extreme_row = grid_iterator.clone().find(|&other_cell| {
            other_cell.address.x == cell.address.x + 2 && other_cell.address.y == cell.address.y
        });
        match (middle_row, extreme_row) {
            (Some(middle), Some(extreme)) => relative_cells_to_eval.push((middle, extreme)),
            (_, _) => {}
        }

        let middle_diagonal_lr = grid_iterator.clone().find(|&other_cell| {
            other_cell.address.x == cell.address.x + 1 && other_cell.address.y == cell.address.y + 1
        });
        let extreme_diagonal_lr = grid_iterator.clone().find(|&other_cell| {
            other_cell.address.x == cell.address.x + 2 && other_cell.address.y == cell.address.y + 2
        });
        match (middle_diagonal_lr, extreme_diagonal_lr) {
            (Some(middle), Some(extreme)) => relative_cells_to_eval.push((middle, extreme)),
            (_, _) => {}
        }

        if cell.address.x >= 2 {
            //Because tictactoe needs at least 3 cells lined, to check diagonal from right to left we need to ensure that at least we are in col 3.
            let middle_diagonal_rl = grid_iterator.clone().find(|&other_cell| {
                other_cell.address.x == cell.address.x - 1
                    && other_cell.address.y == cell.address.y + 1
            });
            let extreme_diagonal_rl = grid_iterator.clone().find(|&other_cell| {
                other_cell.address.x == cell.address.x - 2
                    && other_cell.address.y == cell.address.y + 2
            });
            match (middle_diagonal_rl, extreme_diagonal_rl) {
                (Some(middle), Some(extreme)) => relative_cells_to_eval.push((middle, extreme)),
                (_, _) => {}
            }
        }

        let middle_vertical = grid_iterator.clone().find(|&other_cell| {
            other_cell.address.x == cell.address.x && other_cell.address.y == cell.address.y + 1
        });
        let extreme_vertical = grid_iterator.clone().find(|&other_cell| {
            other_cell.address.x == cell.address.x && other_cell.address.y == cell.address.y + 2
        });
        match (middle_vertical, extreme_vertical) {
            (Some(middle), Some(extreme)) => relative_cells_to_eval.push((middle, extreme)),
            (_, _) => {}
        }

        for (middle, extreme) in relative_cells_to_eval {
            if exist_winning_move(cell, middle, extreme) {
                return true;
            };
        }
    }
    // Check draw
    for cell in grid {
        match cell.state {
            State::Empty => return false,
            _ => break,
        }
    }
    false
}

fn get_player_input(axis: String) -> Result<u8, Error> {
    println!("Please, enter {} coordinate", axis);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let value: u8 = match input.trim().parse() {
        Ok(0) | Err(_) => {
            println!("The input for coordinates are invalid, please, ingress a valid value");
            return get_player_input(axis);
        }
        Ok(number) => number,
    };
    Ok(value)
}

fn get_player_choice() -> Result<Coordinate, Error> {
    let x = get_player_input("x".into())?;
    let y = get_player_input("y".into())?;
    let coordinate = Coordinate::new(x - 1, y - 1);
    Ok(coordinate)
}

fn change_turn(turn_owner: State) -> Result<State, Error> {
    match turn_owner {
        State::X => Ok(State::O),
        State::O => Ok(State::X),
        _ => Err(Error::new("Can't change turns".to_string())),
    }
}

fn main() -> Result<(), Error> {
    let mut grid: Vec<Cell> = vec![];

    for y in 0..ROWS {
        for x in 0..COLS {
            grid.push(Cell::new(x as u8, y as u8));
        }
    }
    let mut turn_owner = State::X;
    show_grid(&grid);
    while !is_gameover(grid.clone()) {
        let coordinate = get_player_choice()?;
        let grid_position = grid
            .iter()
            .position(|&cell| cell.address == coordinate && cell.state == State::Empty);
        match grid_position {
            Some(index) => {
                if grid[index].state == State::Empty && turn_owner != State::Empty {
                    grid[index].state = turn_owner
                }
                turn_owner = change_turn(turn_owner)?;
            }
            None => println!(
                "{},{} is not a valid coordinate for this grid.",
                coordinate.x + 1,
                coordinate.y + 1
            ),
        }
        show_grid(&grid);
    }
    Ok(())
}
