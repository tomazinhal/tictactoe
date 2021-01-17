# Tic Tac Toe in Rust

It consists of a 3x3 grid.

Two players (one playing as X, another playing as O) take turns
playing in the grid. They can only play where a piece has not been placed.

Game ends when X or O get 3 in line, or when all spaces are filled.

## Implementation

### Grid
The grid is 3x3 cells, so 9 in total.
|1|2|3|

|4|5|6|

|7|8|9|

### Cell
Each cell holds a player's value or nothing.

| | or |X| or |O|

### Rules
Game ends as soon as there are 3X or 3O in the same line (diagonals included)
or when all spaces are filled (in this case will be a draw).

A player can only play on an empty cell.

# NOTES
Difference between Clone and Copy in Rust -> https://github.com/rust-lang/rust/blob/2e6eaceedeeda764056eb0e2134735793533770d/src/libcore/marker.rs#L272

To use binary comparison between State -> https://doc.rust-lang.org/std/cmp/trait.PartialEq.html

