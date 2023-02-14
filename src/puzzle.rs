#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct Puzzle(Vec<crate::tube::Tube>);

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

impl Puzzle {
    pub fn new(size: usize) -> Self {
        let mut tubes = vec![crate::tube::Tube::default(); size - 2];
        // The other two tubes will always be empty
        tubes.push(crate::tube::Tube::empty());
        tubes.push(crate::tube::Tube::empty());
        Self(tubes)
    }

    pub fn pour(&mut self, from: usize, to: usize) -> bool {
        if self.0[from].cannot_pour_to(self.0[to]) {
            return false;
        }

        let mut to_tube = self.0[to];
        // TODO: Make this a Result
        self.0[from].pour_to(&mut to_tube);
        self.0[to] = to_tube;
        true
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
