use constants::N;
use std::fmt::Display;

mod constants;
mod util;

// TODO reduce mutability.

// TODO impl Deref or just named struct.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Board(pub [[usize; N]; N]);

// TODO: make beautiful
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO simplify
        for i in 0..N {
            for j in 0..N {
                write!(f, "{} ", self.0[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn bktrk(board: &mut Board, mut row: usize, mut col: usize) -> bool {
    if row == N - 1 && col == N {
        return true;
    }

    if col == N {
        // TODO remove bananas
        row += 1;
        col = 0
    }

    if board.0[row][col] > 0 {
        return bktrk(board, row, col + 1);
    }

    for num in 1..=N {
        if util::valid_pos(board, row, col, num) {
            board.0[row][col] = num;

            if bktrk(board, row, col + 1) {
                return true;
            }
        }
        board.0[row][col] = 0;
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::{bktrk, Board};

    #[test]
    fn it_works() {
        let mut board = Board([
            [3, 0, 6, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ]);

        let expected = Board([
            [3, 1, 6, 5, 7, 8, 4, 9, 2],
            [5, 2, 9, 1, 3, 4, 7, 6, 8],
            [4, 8, 7, 6, 2, 9, 5, 3, 1],
            [2, 6, 3, 4, 1, 5, 9, 8, 7],
            [9, 7, 4, 8, 6, 3, 1, 2, 5],
            [8, 5, 1, 7, 9, 2, 6, 4, 3],
            [1, 3, 8, 9, 4, 7, 2, 5, 6],
            [6, 9, 2, 3, 5, 1, 8, 7, 4],
            [7, 4, 5, 2, 8, 6, 3, 1, 9],
        ]);
        assert!(bktrk(&mut board, 0, 0));
        assert_eq!(expected, board);
    }
}
