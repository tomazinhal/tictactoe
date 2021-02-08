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
        match (i + 1) % 3 {
            0 => println!("|{}|", cell.state),
            _ => print!("|{}", cell.state),
        }
    }
}

fn is_gameover(grid: Vec<Cell>) -> bool {
    // check rows and columns
    for i in 0..2 {
        let row = 3 * i;
        if (
            // rows
            grid[0 + row].state == grid[1 + row].state
        ) && (grid[0 + row].state == grid[2 + row].state)
        {
            match grid[0 + row].state {
                State::Empty => continue,
                _ => return true,
            }
        }
        if (
            // columns
            grid[0 + i].state == grid[3 + i].state
        ) && (grid[0 + i].state == grid[6 + i].state)
        {
            match grid[0 + i].state {
                State::Empty => continue,
                _ => return true,
            }
        }
    }
    // check draw
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
