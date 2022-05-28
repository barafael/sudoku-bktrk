use crate::Board;

pub fn valid_pos(board: &Board, row: usize, col: usize, value: usize) -> bool {
    value == 0
        || (!row_contains(board, row, value)
            && !col_contains(board, col, value)
            && !square_contains(board, row, col, value))
}

pub fn row_contains(board: &Board, row_index: usize, value: usize) -> bool {
    for col_index in 0..9 {
        if board.0[row_index][col_index] == value {
            return true;
        }
    }
    false
}

pub fn col_contains(board: &Board, col_index: usize, value: usize) -> bool {
    for row_index in 0..9 {
        if board.0[row_index][col_index] == value {
            return true;
        }
    }
    false
}

pub fn square_contains(board: &Board, row: usize, col: usize, value: usize) -> bool {
    let start_row = row - row % 3;
    let start_col = col - col % 3;
    for i in 0..3 {
        for j in 0..3 {
            if board.0[start_row + i][start_col + j] == value {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod test {
    use crate::{
        util::{col_contains, square_contains},
        Board,
    };

    fn initial() -> Board {
        Board([
            [0, 2, 0, 4, 0, 0, 7, 0, 0],
            [7, 0, 0, 0, 0, 6, 0, 0, 8],
            [0, 8, 3, 0, 0, 0, 0, 0, 1],
            [0, 0, 2, 6, 0, 0, 0, 0, 0],
            [0, 5, 0, 0, 0, 0, 0, 7, 0],
            [0, 0, 0, 0, 0, 3, 9, 0, 0],
            [9, 0, 0, 0, 0, 0, 8, 3, 0],
            [3, 0, 0, 5, 0, 0, 0, 0, 7],
            [0, 0, 1, 0, 0, 4, 0, 6, 0],
        ])
    }

    #[test]
    fn square_contains_works() {
        let mut board = initial();
        board.0[0][2] = 1;
        board.0[2][0] = 3;
        board.0[1][1] = 5;

        assert!(square_contains(&board, 1, 1, 1));
        assert!(square_contains(&board, 2, 2, 3));
        assert!(square_contains(&board, 0, 0, 5));
        assert!(!square_contains(&board, 2, 2, 4));
    }

    #[test]
    fn column_contains() {
        let mut initial = initial();
        initial.0[1][1] = 1;
        initial.0[8][1] = 3;
        initial.0[3][1] = 5;
        assert!(col_contains(&initial, 1, 3));
        assert!(col_contains(&initial, 1, 5));
        assert!(col_contains(&initial, 8, 7));
        assert!(!col_contains(&initial, 7, 1));
    }
}
