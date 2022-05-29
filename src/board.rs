use derive_more::{Deref, DerefMut};
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
pub struct Board(pub(crate) [[usize; N]; N]);

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

#[cfg(test)]
mod test {
    use super::*;

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn deser_roundtrip(board in any::<Board>()) {
            let json = serde_json::to_string(&board).unwrap();
            let deser: Board = serde_json::from_str(&json).unwrap();
            assert_eq!(board, deser);
        }
    }
}
