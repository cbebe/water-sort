use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Puzzle(Vec<crate::tube::Tube>);

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.0.len();
        let mid = if size % 2 == 0 {
            size / 2
        } else {
            size / 2 + 1
        };
        self.print_row(f, 0, mid)?;
        f.write_str("-------------------------\n")?;
        self.print_row(f, mid, size)?;
        Ok(())
    }
}

impl Puzzle {
    pub fn new(size: usize) -> Self {
        let mut tubes = vec![crate::tube::Tube::default(); size - 2];
        // The other two tubes will always be empty
        tubes.push(crate::tube::Tube::empty());
        tubes.push(crate::tube::Tube::empty());
        Self(tubes)
    }

    pub fn set_tube(&mut self, tube: usize, idx: usize, state: crate::tube::State) {
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
        f.write_fmt(format_args!("{:2}", " "))?;
        for tube in start..end - 1 {
            f.write_fmt(format_args!("{tube:3}   "))?;
        }
        f.write_fmt(format_args!("{:3}\n", end - 1))?;
        for row in 0..4 {
            f.write_fmt(format_args!("{row} "))?;
            for tube in start..end - 1 {
                f.write_fmt(format_args!("|{}| ", self.0[tube].get(row)))?;
            }
            f.write_fmt(format_args!("|{}|\n", self.0[end - 1].get(row)))?;
        }
        Ok(())
    }
}
