extern crate test;

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::PlaceValue::*;
    use super::test::Bencher;

    fn run_compliance_tests(eval_winner: fn (board: &Board) -> Option<PlaceValue>) {
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

    #[test]
    fn test_bit_wise_eval_winner() {
        run_compliance_tests(bit_strategy::eval_winner);
    }

    #[bench]
    fn bench_bit_wise_eval_winner(b: &mut Bencher) {
        b.iter(|| run_compliance_tests(bit_strategy::eval_winner));
    }

    #[test]
    fn test_loop_eval_winner() {
        run_compliance_tests(loop_strategy::eval_winner);
    }

    #[bench]
    fn bench_loop_eval_winner(b: &mut Bencher) {
        b.iter(|| run_compliance_tests(loop_strategy::eval_winner));
    }
}
