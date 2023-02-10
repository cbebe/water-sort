use crate::state::State;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Tube {
    t: [State; 4],
    sticky: bool,
}

impl Tube {
    pub const fn empty() -> Self {
        Self {
            t: [State::Empty, State::Empty, State::Empty, State::Empty],
            sticky: false,
        }
    }

    pub fn set(&mut self, idx: usize, state: State) {
        self.t[idx] = state;
    }

    pub const fn get(self, idx: usize) -> State {
        self.t[idx]
    }

    fn num_to_pour(self) -> usize {
        let (top, num_free) = (self.top(), self.num_free());
        if let State::Water(w) = top {
            let mut to_pour = 0;
            for i in num_free..4 {
                if matches!(self.t[i], State::Water(other) if other == w) {
                    to_pour += 1;
                }
            }
            to_pour
        } else {
            0
        }
    }

    const fn num_free(self) -> usize {
        use State::Empty as E;
        match (self.t[0], self.t[1], self.t[2], self.t[3]) {
            (E, E, E, E) => 4,
            (E, E, E, _) => 3,
            (E, E, _, _) => 2,
            (E, _, _, _) => 1,
            (_, _, _, _) => 0,
        }
    }

    pub fn cannot_pour_to(self, other: Self) -> bool {
        !self.can_pour_to(other)
    }

    pub fn can_pour_to(self, other: Self) -> bool {
        match (self.top(), other.top()) {
            (State::Empty, _) => false,
            (_, State::Empty) => true,
            (a, b) if a == b => self.num_to_pour() <= other.num_free(),
            (_, _) => false,
        }
    }

    const fn top(self) -> State {
        use State::Empty as E;
        match (self.t[0], self.t[1], self.t[2], self.t[3]) {
            (E, E, E, s) | (E, E, s, _) | (E, s, _, _) | (s, _, _, _) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::State::{Empty, Unknown, Water};
    use crate::water::Water::Blue;

    #[test]
    fn test_num_to_pour() {
        let mut a = Tube::empty();

        a.set(0, Water(Blue));
        a.set(1, Water(Blue));
        a.set(2, Water(Blue));
        a.set(3, Water(Blue));

        assert_eq!(a.num_to_pour(), 4);
    }

    #[test]
    fn test_num_free_full() {
        let mut a = Tube::empty();

        a.set(0, Water(Blue));
        a.set(1, Water(Blue));
        a.set(2, Water(Blue));
        a.set(3, Water(Blue));

        assert_eq!(a.num_free(), 0);
    }

    #[test]
    fn test_num_free_empty() {
        assert_eq!(Tube::empty().num_free(), 4);
    }

    #[test]
    fn test_num_free_one() {
        let mut a = Tube::empty();

        a.set(1, Water(Blue));
        a.set(2, Water(Blue));
        a.set(3, Water(Blue));

        assert_eq!(a.num_free(), 1);
    }

    #[test]
    fn test_can_pour_to_empty() {
        let (mut a, b) = (Tube::empty(), Tube::empty());
        a.set(3, Water(Blue));
        assert!(a.can_pour_to(b));
    }

    #[test]
    fn test_can_pour_to_one() {
        use crate::water::Water::Blue;
        let (mut a, mut b) = (Tube::empty(), Tube::empty());
        a.set(3, Water(Blue));
        b.set(3, Water(Blue));
        assert!(a.can_pour_to(b));
    }

    #[test]
    fn test_cannot_pour_to_full() {
        use crate::water::Water::Blue;
        let (mut a, mut b) = (Tube::empty(), Tube::empty());
        a.set(3, Water(Blue));

        b.set(0, Water(Blue));
        b.set(1, Water(Blue));
        b.set(2, Water(Blue));
        b.set(3, Water(Blue));

        assert!(a.cannot_pour_to(b));
    }

    /// Technically, in the game, you can still
    /// pour to the brim and have some left over.
    /// However, the state wouldn't really be
    /// any different, so we just won't count that.
    #[test]
    fn test_cannot_pour_to_overflowing() {
        let (mut a, mut b) = (Tube::empty(), Tube::empty());
        a.set(2, Water(Blue));
        a.set(3, Water(Blue));
        b.set(0, Empty);
        b.set(1, Water(Blue));
        b.set(2, Unknown);
        b.set(3, Unknown);
        assert!(a.cannot_pour_to(b));
    }
}
