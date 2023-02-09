use std::fmt::Display;

use ansi_term::{Colour, Style};

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
enum Water {
    Blue,
    Brown,
    Cyan,
    Green,
    Grey,
    Lime,
    Olive,
    Orange,
    Pink,
    Purple,
    Red,
    Yellow,
}

impl Water {
    fn get_colour(&self) -> Colour {
        use Colour::RGB;
        match self {
            Water::Blue => RGB(58, 46, 195),
            Water::Brown => RGB(126, 74, 7),
            Water::Cyan => RGB(84, 163, 228),
            Water::Green => RGB(17, 101, 51),
            Water::Grey => RGB(99, 100, 101),
            Water::Lime => RGB(98, 214, 124),
            Water::Olive => RGB(120, 150, 15),
            Water::Orange => RGB(232, 140, 66),
            Water::Pink => RGB(234, 94, 123),
            Water::Purple => RGB(113, 43, 147),
            Water::Red => RGB(197, 42, 35),
            Water::Yellow => RGB(241, 217, 87),
        }
    }

    fn style(&self) -> Style {
        Style::new().on(self.get_colour())
    }
}

impl Display for Water {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.style().paint("   ").fmt(f)
    }
}

#[derive(Default, Clone, Copy, Debug)]
#[allow(dead_code)]
enum State {
    #[default]
    Unknown,
    Sticky(Water),
    NonStick(Water),
    Empty,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Unknown => Style::new()
                .on(Colour::Black)
                .fg(Colour::White)
                .paint(" ? ")
                .fmt(f),
            State::Sticky(w) => w.fmt(f),
            State::NonStick(w) => w.style().paint(" s ").fmt(f),
            State::Empty => Style::new()
                .on(Colour::Black)
                .fg(Colour::White)
                .paint("   ")
                .fmt(f),
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
struct Tube([State; 4]);

impl Tube {
    const fn empty() -> Self {
        Self([State::Empty, State::Empty, State::Empty, State::Empty])
    }
}

#[derive(Default, Debug)]
struct Puzzle(Vec<Tube>);

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.0.len();
        let (top, bottom) = if let 0 = size % 2 {
            (size / 2, size / 2)
        } else {
            (size / 2 + 1, size / 2)
        };
        for row in 0..4 {
            for tube in 0..top - 1 {
                f.write_fmt(format_args!("|{}| ", self.0[tube].0[row]))?
            }
            f.write_fmt(format_args!("|{}|\n", self.0[top - 1].0[row]))?
        }
        f.write_str("-------------------------\n")?;
        for row in 0..4 {
            for tube in bottom..size - 1 {
                f.write_fmt(format_args!("|{}| ", self.0[tube].0[row]))?
            }
            f.write_fmt(format_args!("|{}|\n", self.0[size - 1].0[row]))?
        }
        Ok(())
    }
}

impl Puzzle {
    fn new(size: usize) -> Self {
        let mut tubes = vec![Tube::default(); size - 2];
        // The other two tubes will always be empty
        tubes.push(Tube::empty());
        tubes.push(Tube::empty());
        Self(tubes)
    }
}

fn main() {
    let mut puzzle = Puzzle::new(12);
    puzzle.0[0].0[3] = State::Sticky(Water::Blue);
    println!("{}", puzzle);
}
