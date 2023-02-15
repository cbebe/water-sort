#[derive(
    Default, Clone, Copy, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub enum State {
    #[default]
    Unknown,
    Water(crate::water::Water),
    Empty,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ansi_term::{
            Colour::{Black as B, White as W},
            Style,
        };
        match self {
            Self::Unknown => Style::new().on(B).fg(W).paint(" ? ").fmt(f),
            Self::Water(w) => w.fmt(f),
            Self::Empty => Style::new().on(B).fg(W).paint("   ").fmt(f),
        }
    }
}
