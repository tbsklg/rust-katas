fn main() {
    println!("{}", is_solved(&[&[1, 1, 2], &[0, 2, 2], &[1, 1, 1]]))
}

fn is_solved(board: &[&[u8; 3]; 3]) -> i8 {
    player_1_wins(board)
        .or_else(|| player_2_wins(board))
        .or_else(|| empty_spots(board))
        .unwrap_or(0)
}

fn empty_spots(board: &[&[u8; 3]; 3]) -> Option<i8> {
    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] == 0 {
                return Option::Some(-1);
            }
        }
    }
    Option::None
}

fn player_1_wins(board: &[&[u8; 3]; 3]) -> Option<i8> {
    match board {
        &[&[1, _, _], &[_, 1, _], &[_, _, 1]] => Option::Some(1),
        &[&[_, _, 1], &[_, 1, _], &[1, _, _]] => Option::Some(1),
        &[&[1, _, _], &[1, _, _], &[1, _, _]] => Option::Some(1),
        &[&[_, 1, _], &[_, 1, _], &[_, 1, _]] => Option::Some(1),
        &[&[_, _, 1], &[_, _, 1], &[_, _, 1]] => Option::Some(1),
        &[&[1, 1, 1], &[_, _, _], &[_, _, _]] => Option::Some(1),
        &[&[_, _, _], &[1, 1, 1], &[_, _, _]] => Option::Some(1),
        &[&[_, _, _], &[_, _, _], &[1, 1, 1]] => Option::Some(1),
        _ => Option::None,
    }
}

fn player_2_wins(board: &[&[u8; 3]; 3]) -> Option<i8> {
    match board {
        &[&[2, _, _], &[_, 2, _], &[_, _, 2]] => Option::Some(2),
        &[&[_, _, 2], &[_, 2, _], &[2, _, _]] => Option::Some(2),
        &[&[2, 2, 2], &[_, _, _], &[_, _, _]] => Option::Some(2),
        &[&[2, _, _], &[2, _, _], &[2, _, _]] => Option::Some(2),
        &[&[_, 2, _], &[_, 2, _], &[_, 2, _]] => Option::Some(2),
        &[&[_, _, 2], &[_, _, 2], &[_, _, 2]] => Option::Some(2),
        &[&[_, _, _], &[2, 2, 2], &[_, _, _]] => Option::Some(2),
        &[&[_, _, _], &[_, _, _], &[2, 2, 2]] => Option::Some(2),
        _ => Option::None,
    }
}

#[cfg(test)]
mod tests {
    use super::is_solved;

    fn dotest(board: &[&[u8; 3]; 3], expected: i8) {
        let actual = is_solved(board);
        assert!(
            actual == expected,
            "With board = {board:?}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        for (board, expected) in [
            ([&[0, 0, 1], &[0, 1, 2], &[2, 1, 0]], -1),
            ([&[1, 1, 1], &[0, 2, 2], &[0, 0, 0]], 1),
            ([&[2, 1, 2], &[2, 1, 1], &[1, 1, 2]], 1),
            ([&[2, 1, 2], &[2, 1, 1], &[1, 2, 1]], 0),
        ] {
            dotest(&board, expected);
        }
    }
}
