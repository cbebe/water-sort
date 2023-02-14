#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone)]
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

pub struct ValidMoves(pub Vec<(usize, usize)>);

impl ValidMoves {
    pub fn to_owned(self) -> Vec<(usize, usize)> {
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

impl Puzzle {
    pub fn new(size: usize) -> Self {
        let mut tubes = vec![crate::tube::Tube::default(); size - 2];
        // The other two tubes will always be empty
        tubes.push(crate::tube::Tube::empty());
        tubes.push(crate::tube::Tube::empty());
        Self(tubes)
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
                    valid.push((i, j));
                }
            }
        }
        ValidMoves(valid)
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
