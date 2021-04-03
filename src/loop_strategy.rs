use super::*;
use super::PlaceValue::*;

const WINNING_POSITIONS: [[u8;9];8] = [
    [1, 1, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 1, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 1, 1],
    [1, 0, 0, 1, 0, 0, 1, 0, 0],
    [0, 1, 0, 0, 1, 0, 0, 1, 0],
    [0, 0, 1, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 0, 1, 0, 0, 0, 1],
    [0, 0, 1, 0, 1, 0, 1, 0, 0]
];

fn get_row_from_index(board: &Board, i: usize) -> [PlaceValue;3] {
    if i >= 6 {
        return board.row3;
    }
    if i >= 3 {
        return board.row2;
    }
    return board.row1;
}

fn value_on_board(board: &Board, i: usize) -> PlaceValue {
    let row = get_row_from_index(board, i);
    let index = i % 3;
    row[index]
}

pub fn eval_winner(board: &Board) -> Option<PlaceValue> {
    // For each winning combo
    for combo in &WINNING_POSITIONS {
        let mut x_matches = 0;
        let mut o_matches = 0;
        // iterate through each value in the winning combo
        for i in 0..combo.len() {
            // if the position is not a winning value, ignore it.
            if combo[i] == 0 {
                continue;
            }
            // get the value of that position on the board
            match value_on_board(&board, i) {
                // if it's X, increment x matches
                X => x_matches += 1,
                // if it's O, increment o matches
                O => o_matches += 1,
                _ => ()
            }
        }
        // if X has three matches, it's a winner
        if x_matches == 3 {
            return Some(X)
        }
        // likewise for O
        if o_matches == 3 {
            return Some(O)
        }
    }
    // No winner.
    None
}
