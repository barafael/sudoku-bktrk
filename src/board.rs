use derive_more::{Deref, DerefMut};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::constants::N;
use std::fmt::Display;

// TODO impl Deref or just named struct.
#[derive(
    Debug,
    Copy,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Deref,
    DerefMut,
)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct Board(pub [[usize; N]; N]);

/*
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
*/

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hor_line = Some("------------\n".to_string());
        hor_line
            .iter()
            .cloned()
            .chain(
                self.into_iter()
                    .flatten()
                    .map(|n| format!("{n}"))
                    .chunks(27)
                    .into_iter()
                    .map(|chunk| {
                        chunk
                            .chunks(9)
                            .into_iter()
                            .map(|line| {
                                line.into_iter()
                                    .chunks(3)
                                    .into_iter()
                                    .map(|chunk| chunk.chain(Some("|".to_string())))
                                    .flatten()
                                    .chain(Some("\n".to_string()))
                                    .collect_vec()
                            })
                            .flatten()
                            .chain(hor_line.clone())
                            .collect_vec()
                    })
                    .flatten(),
            )
            .map(|v| write!(f, "{}", v))
            .collect::<Result<(), _>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn displays() {
        let board = Board::default();
        println!("{board}");
    }

    proptest! {
        #[test]
        fn deser_roundtrip(board in any::<Board>()) {
            let json = serde_json::to_string(&board).unwrap();
            let deser: Board = serde_json::from_str(&json).unwrap();
            assert_eq!(board, deser);
        }
    }
}
