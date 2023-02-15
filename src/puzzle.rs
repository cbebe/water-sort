use std::hash::Hash;

use crate::state::State;

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Puzzle(Vec<crate::tube::Tube>);

impl Hash for Puzzle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in self.0.iter() {
            i.hash(state);
        }
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.0.len();
        let mid = if size % 2 == 0 {
            size / 2
        } else {
            size / 2 + 1
        };
        self.print_row(f, 0, mid)?;
        writeln!(f, "-------------------------")?;
        self.print_row(f, mid, size)?;
        Ok(())
    }
}

pub struct ValidMoves(pub Vec<(u8, u8)>);

impl ValidMoves {
    // rustc: destructor of `ValidMoves` cannot be evaluated at compile-time
    // the destructor for this type cannot be evaluated in constant functions
    #[allow(clippy::missing_const_for_fn)]
    pub fn get(self) -> Vec<(u8, u8)> {
        self.0
    }
}

impl std::fmt::Display for ValidMoves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|i| format!("({}, {})", i.0, i.1))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}

#[derive(Debug)]
pub struct InvalidMove {
    pub from: usize,
    pub to: usize,
}

impl Puzzle {
    pub fn new(size: usize) -> Self {
        let mut tubes = vec![crate::tube::Tube::default(); size - 2];
        // The other two tubes will always be empty
        tubes.push(crate::tube::Tube::empty());
        tubes.push(crate::tube::Tube::empty());
        Self(tubes)
    }

    pub fn has_unknown(&self) -> bool {
        self.0.iter().any(|t| t.top() == State::Unknown)
    }

    pub fn is_solved(&self) -> bool {
        self.0
            .iter()
            .all(|t| t.num_to_pour() == 4 || t.num_free() == 4)
    }

    pub fn valid_moves(&self) -> ValidMoves {
        let mut valid = vec![];
        for i in 0..self.0.len() {
            for j in 0..self.0.len() {
                if i == j {
                    continue;
                }
                if self.0[i].can_pour_to(self.0[j]) && self.0[i].num_to_pour() != 4 {
                    valid.push((i as u8, j as u8));
                }
            }
        }
        ValidMoves(valid)
    }

    pub fn pour(&mut self, from: usize, to: usize) -> Result<(), InvalidMove> {
        if self.0[from].cannot_pour_to(self.0[to]) {
            return Err(InvalidMove { from, to });
        }

        let mut to_tube = self.0[to];
        // TODO: Make this a Result
        self.0[from].pour_to(&mut to_tube);
        self.0[to] = to_tube;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn set_whole_tube(&mut self, tube: usize, state: [crate::state::State; 4]) {
        self.0[tube].set_tube(state);
    }

    pub fn set_tube(&mut self, tube: usize, idx: usize, state: crate::state::State) {
        self.0[tube].set(idx, state);
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn reset(&mut self, p: Self) {
        self.0 = p.0;
    }

    fn print_row(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        start: usize,
        end: usize,
    ) -> std::fmt::Result {
        write!(f, "{:2}", " ")?;
        for tube in start..end - 1 {
            write!(f, "{tube:3}   ")?;
        }
        writeln!(f, "{:3}", end - 1)?;
        for row in 0..4 {
            write!(f, "{row} ")?;
            for tube in start..end - 1 {
                write!(f, "|{}| ", self.0[tube].get(row))?;
            }
            writeln!(f, "|{}|", self.0[end - 1].get(row))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod puzzle_test {
    use super::*;
    use crate::state::State::{Empty, Water};
    use crate::water::Water::Red;
    #[test]
    fn test_is_solved() {
        let mut p = Puzzle::new(3);
        p.set_tube(0, 0, Empty);
        p.set_tube(0, 1, Empty);
        p.set_tube(0, 2, Water(Red));
        p.set_tube(0, 3, Water(Red));
        p.set_tube(1, 0, Empty);
        p.set_tube(1, 1, Empty);
        p.set_tube(1, 2, Water(Red));
        p.set_tube(1, 3, Water(Red));
        assert!(!p.is_solved());
        p.pour(0, 1).unwrap();
        assert!(p.is_solved());
    }
}
