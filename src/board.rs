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
                    .flat_map(|chunk| {
                        chunk
                            .chunks(9)
                            .into_iter()
                            .flat_map(|line| {
                                line.into_iter()
                                    .chunks(3)
                                    .into_iter()
                                    .flat_map(|chunk| chunk.chain(Some("|".to_string())))
                                    .chain(Some("\n".to_string()))
                                    .collect_vec()
                            })
                            .chain(hor_line.clone())
                            .collect_vec()
                    }),
            )
            .try_for_each(|v| write!(f, "{}", v))
    }
}
*/

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hor_line = "|-----------------------|".to_string();
        writeln!(f, "{hor_line}")?;
        for third in &self
            .iter()
            .flatten()
            .map(|num| char::from_digit(*num as u32, 10).expect("digit conversion failed"))
            .chunks(27)
        {
            for line in third.chunks(9).into_iter() {
                write!(f, "| ")?;
                for triplet in line.chunks(3).into_iter() {
                    write!(f, "{} | ", triplet.format(" "))?;
                }
                writeln!(f)?;
            }
            writeln!(f, "{hor_line}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::example;
    use proptest::prelude::*;

    #[test]
    fn displays() {
        let board = example();
        let formatted = format!("{board}");
        let expected = r##"|-----------------------|
| 0 2 0 | 4 0 0 | 7 0 0 | 
| 7 0 0 | 0 0 6 | 0 0 8 | 
| 0 8 3 | 0 0 0 | 0 0 1 | 
|-----------------------|
| 0 0 2 | 6 0 0 | 0 0 0 | 
| 0 5 0 | 0 0 0 | 0 7 0 | 
| 0 0 0 | 0 0 3 | 9 0 0 | 
|-----------------------|
| 9 0 0 | 0 0 0 | 8 3 0 | 
| 3 0 0 | 5 0 0 | 0 0 7 | 
| 0 0 1 | 0 0 4 | 0 6 0 | 
|-----------------------|
"##;
        assert_eq!(expected, formatted);
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
