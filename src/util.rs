use std::convert::identity;

use crate::Board;

const SQUARE_SIZE: usize = 3;

pub fn valid_pos(board: &Board, row: usize, col: usize, value: usize) -> bool {
    value == 0
        || (!row_contains(board, row, value)
            && !col_contains(board, col, value)
            && !square_contains(board, row, col, value))
}

pub fn row_contains(board: &Board, row_index: usize, value: usize) -> bool {
    board.0[row_index].into_iter().any(|v| v == value)
}

pub fn col_contains(board: &Board, col_index: usize, value: usize) -> bool {
    board.0.into_iter().any(|row| row[col_index] == value)
}

// TODO simplify iterator spaghet
pub fn square_contains(board: &Board, row: usize, col: usize, value: usize) -> bool {
    let start_row = row - row % 3;
    let start_col = col - col % 3;
    (start_row..start_row + SQUARE_SIZE)
        .into_iter()
        .map(|i| {
            (start_col..start_col + SQUARE_SIZE)
                .into_iter()
                .map(move |j| (i, j))
                .any(|(i, j)| board.0[i][j] == value)
        })
        .any(identity)
}

#[cfg(test)]
mod test {
    use crate::{
        util::{col_contains, row_contains, square_contains},
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
        assert!(!col_contains(&initial, 7, 1));
    }

    #[test]
    fn row_contains_works() {
        let mut initial = initial();
        initial.0[1][1] = 1;
        initial.0[1][8] = 3;
        initial.0[1][3] = 5;
        assert!(row_contains(&initial, 1, 3));
        assert!(row_contains(&initial, 1, 5));
        assert!(!row_contains(&initial, 7, 1));
    }
}
