use ansi_term::{Colour, Style};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum State {
    #[default]
    Unknown,
    Sticky(crate::water::Water),
    NonStick(crate::water::Water),
    Empty,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => Style::new()
                .on(Colour::Black)
                .fg(Colour::White)
                .paint(" ? ")
                .fmt(f),
            Self::Sticky(w) => w.fmt(f),
            Self::NonStick(w) => w.style().paint(" s ").fmt(f),
            Self::Empty => Style::new()
                .on(Colour::Black)
                .fg(Colour::White)
                .paint("   ")
                .fmt(f),
        }
    }
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Tube([State; 4]);

impl Tube {
    pub const fn empty() -> Self {
        Self([State::Empty, State::Empty, State::Empty, State::Empty])
    }

    pub fn set(&mut self, idx: usize, state: State) {
        self.0[idx] = state;
    }

    pub const fn get(self, idx: usize) -> State {
        self.0[idx]
    }

    #[allow(dead_code)]
    const fn top(self) -> State {
        use State::Empty;
        match (self.0[0], self.0[1], self.0[2], self.0[3]) {
            (s, Empty, Empty, Empty) | (_, s, Empty, Empty) | (_, _, s, Empty) | (_, _, _, s) => s,
        }
    }

    #[allow(dead_code)]
    const fn has_space(self) -> bool {
        matches!(self.0[3], State::Empty)
    }
}
