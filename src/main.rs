use std::fmt;

const ROWS: usize = 3;
const COLS: usize = 3;

#[derive(Debug, Copy, Clone)]
enum State {
    Empty,
    X,
    O
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
        match self {
            other => true,
            _ => false
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
            state: State::Empty,
            x: x,
            y: y
        }
    }
}

fn show_grid(grid: &Vec<Cell>) {
    for (i, cell) in grid.iter().enumerate() {
        match (i + 1)  % 3 {
            0 => println!("|{}|", cell.state),
            _ => print!("|{}", cell.state),
        }
    }
}

fn is_gameover(grid: &Vec<Cell>) -> bool {
    // bruh.jpeg
    // check rows and columns
    for i in 0..2 {
        let row = 3 * i;
        if ( // rows
            grid[0 + row].state == grid[1 + row].state
            ) == (
            grid[0 + row].state == grid[2 + row].state
            ) {
            match grid[0 + row].state {
                State::Empty => continue,
                _ => return true,
            }
        }
        if ( // columns
            grid[0 + i].state == grid[3 + i].state
            ) == (
            grid[0 + i].state == grid[6 + i].state
            ) {
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

fn play(grid: &Vec<Cell>, x: u8, y: u8, state: State) {
    println!("{:?} is being played on ({}, {})", state, x, y);
    if x > 3 || y > 3{
        println!("Coordinate not valid");
    }
}

fn find(grid: &Vec<Cell>, x: u8, y: u8) -> Result<&Cell, bool> {
    for cell in grid.iter() {
        if cell.x == x && cell.y == y {
            println!("Coordinates found");
        }
        else {
            continue;
        }
        match cell.state {
            State::Empty => return Ok(cell),
            _ => continue,
        }
    }
    return Err(false);
}

fn player_choose(state: State) {
    println!("Player with {} choose a coordinate:", state);
    let choice = String::from("11");
    if choice.len() != 2 {
        return;
    }
    let mut chars = choice.chars();
    // there must be a way to make this better/simpler
    let x: u8 = chars.nth(0).unwrap().to_digit(10).expect("X is not u8") as u8;
    let y: u8 = chars.nth(0).unwrap().to_digit(10).expect("Y is not u8") as u8;
    println!("Player {} chose ({}, {})", state, x, y);
}

fn main () {
    let mut grid: Vec<Cell> = vec![];

    for y in 0..ROWS {
        for x in 0..COLS {
            grid.push(Cell::new(x as u8, y as u8));
        }
    }
    let cell = Cell {x: 1, y: 1, state: State::O};
    let result = find(&grid, cell.x, cell.y);
    match result {
        Ok(found) => println!("Found cell"),
        Err(e) => println!("Not found"),
    }
    player_choose(State::X);
    play(&grid, 2, 2, State::X);
    show_grid(&grid);
    is_gameover(&grid);
}

