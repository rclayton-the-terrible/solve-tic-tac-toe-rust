#![feature(test)]

mod bit_strategy;
mod loop_strategy;
mod tests;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Copy)]
pub enum PlaceValue {
    X,
    O,
    E // Empty
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Board {
    row1: [PlaceValue;3],
    row2: [PlaceValue;3],
    row3: [PlaceValue;3],
}

impl Board {
    #[allow(dead_code)]
    pub fn from(positions: [PlaceValue;9]) -> Board {
        Board {
            row1: [positions[0], positions[1], positions[2]],
            row2: [positions[3], positions[4], positions[5]],
            row3: [positions[6], positions[7], positions[8]],
        }
    }
}

fn main() {
    println!("Demo of finding the winner in tic-tac-toe...");

    use PlaceValue::*;

    let board = Board {
        row1: [X, O, E],
        row2: [O, X, E],
        row3: [E, O, X],
    };

    match bit_strategy::eval_winner(&board) {
        Some(winner) => println!("Winner: {:?}", winner),
        None => println!("No winner yet")
    }
}
