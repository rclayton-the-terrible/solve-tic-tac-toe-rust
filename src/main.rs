#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Copy)]
enum PlaceValue {
    X,
    O,
    E // Empty
}

#[derive(Debug)]
#[derive(Clone)]
struct Board {
    row1: [PlaceValue;3],
    row2: [PlaceValue;3],
    row3: [PlaceValue;3],
}

impl Board {
    #[allow(dead_code)]
    fn from(positions: [PlaceValue;9]) -> Board {
        Board {
            row1: [positions[0], positions[1], positions[2]],
            row2: [positions[3], positions[4], positions[5]],
            row3: [positions[6], positions[7], positions[8]],
        }
    }
}

struct XPosOPos(u16, u16);

// All winning tic-tac-toe positions where the 9 bits align:
// r=row, c=column
// [r1c1 r1c2 r1c3 r2c1 r2c2 r2c3 r3c1 r3c2 r3c3]
// "1" represents a value (either X or O); "0" is not/applicable for
// the winning position.
const WINNING_POSITIONS: [u16;8] = [
    0b_111_000_000,
    0b_000_111_000,
    0b_000_000_111,
    0b_100_100_100,
    0b_010_010_010,
    0b_001_001_001,
    0b_100_010_001,
    0b_001_010_100
];

// Dumb function to select the correct row based on bit index.
fn get_row(board: &Board, i: usize) -> [PlaceValue;3] {
    if i >= 6 {
        return board.row1;
    }
    if i >= 3 {
        return board.row2;
    }
    return board.row3;
}

// Get the correct position in a row based on bit index.
fn get_row_pos(i: usize) -> usize {
    match i % 3 {
        0 => 2,
        1 => 1,
        _ => 0,
    }
}

//  Assuming the left-most bit is row 1/col 1 and right most is row 3/col 3,
//  we will have 9 bits: [r1c1 r1c2 r1c3 r2c1 r2c2 r2c3 r3c1 r3c2 r3c3]
//  So [ X, X, X, E, E, E, O, O, O] will result in the tuple:
// (0b111000000, 0b000000111)
fn board_to_bits(board: &Board) -> XPosOPos {
    // Maintain bit positions for each player (X and O)
    let mut x_bits: u16 = 0;
    let mut o_bits: u16 = 0;
    // Iterate over bit positions
    for i in 0..9 {
        // Translate the bit position to the correct row.
        // row1 = 6-8 (left-most bits)
        // row2 = 3-5
        // row3 = 0-2 (right-most bits)
        let row = get_row(&board, i);
        // Position is in the array, which is inverted i % 3
        // since rows are 3 items long, and the left most bit
        // is the highest index in the remainder.
        let position = get_row_pos(i);
        // Match the value on the board.
        match row[position] {
            // if the value is X, set the i-th bit
            PlaceValue::X => x_bits |= 1 << i,
            // if the valye is O, set the i-th bit
            PlaceValue::O => o_bits |= 1 << i,
            // if the position is empty, do nothing
            _ => (),
        }
    }
    // Return the board state of X and O.
    XPosOPos(x_bits, o_bits)
}

// Converts the board to bits and iterates over
// winning positions.  The first side (X or O) to match
// a winning position is returned as the winner Some(X) or Some(O).
// If there are no winners, None is returned.
fn eval_winner(board: &Board) -> Option<PlaceValue> {
    let XPosOPos(x_bits, o_bits) = board_to_bits(&board);
    for pos in &WINNING_POSITIONS {
        if (pos & x_bits) == *pos {
            return Some(PlaceValue::X);
        }
        if (pos & o_bits) == *pos {
            return Some(PlaceValue::O);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::PlaceValue::*;

    fn assert_positions(positions: u16, pos_hash: &str) {
        assert_eq!(format!("{:09b}", positions), pos_hash);
    }

    #[test]
    fn test_board_to_bits() {
        let b1 = Board::from([X, X, E, O, E, X, E, O, O]);
        let XPosOPos(x1, o1) = board_to_bits(&b1);
        assert_positions(x1, "110001000");
        assert_positions(o1, "000100011");
    }

    #[test]
    fn test_eval_winner() {
        let conditions = [
            (None, Board::from([X, E, X, E, E, O, O, E, O])),
            (Some(X), Board::from([X, X, X, E, E, O, O, E, O])),
            (Some(X), Board::from([X, O, O, E, X, E, O, E, X])),
            (Some(X), Board::from([E, O, X, E, E, X, O, E, X])),
            (Some(O), Board::from([X, X, O, E, O, X, O, E, O])),
            (Some(O), Board::from([E, X, O, E, E, O, O, E, O])),
            (Some(O), Board::from([E, O, X, E, O, E, X, O, X])),
        ];
        for i in 0..conditions.len() {
            let (expected, board) = conditions.get(i).unwrap();
            let res = eval_winner(&board);
            assert_eq!(expected.is_none(), res.is_none());
            if let Some(expected_value) = expected {
                assert!(res.is_some());
                let actual = res.unwrap();
                assert_eq!(*expected_value, actual);
            } else {
                assert!(res.is_none());
            }
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

    match eval_winner(&board) {
        Some(winner) => println!("Winner: {:?}", winner),
        None => println!("No winner yet")
    }
}
