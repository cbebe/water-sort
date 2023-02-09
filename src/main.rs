use ansi_term::{Colour, Style};
use rustyline::{error::ReadlineError, Editor, Result};
use std::fmt::Display;

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
    Unknown,
}

impl From<&&str> for Water {
    fn from(value: &&str) -> Self {
        use Water::*;
        match *value {
            "b" | "bl" | "blue" => Blue,
            "br" | "brown" => Brown,
            "c" | "cy" | "cyan" => Cyan,
            "g" | "green" => Green,
            "gr" | "grey" | "gray" => Grey,
            "l" | "lime" => Lime,
            "ol" | "olive" => Olive,
            "o" | "or" => Orange,
            "p" | "pi" => Pink,
            "pu" | "purple" => Purple,
            "r" | "red" => Red,
            "y" | "yellow" => Yellow,
            unknown => {
                println!("unknown colour: {}", unknown);
                Unknown
            }
        }
    }
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
            Water::Unknown => RGB(255, 255, 255),
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
        let mid = if let 0 = size % 2 {
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
    fn new(size: usize) -> Self {
        let mut tubes = vec![Tube::default(); size - 2];
        // The other two tubes will always be empty
        tubes.push(Tube::empty());
        tubes.push(Tube::empty());
        Self(tubes)
    }

    fn print_row(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        start: usize,
        end: usize,
    ) -> std::fmt::Result {
        f.write_fmt(format_args!("{:2}", " "))?;
        for tube in start..end - 1 {
            f.write_fmt(format_args!("{:3}   ", tube))?;
        }
        f.write_fmt(format_args!("{:3}\n", end - 1))?;
        for row in 0..4 {
            f.write_fmt(format_args!("{} ", row))?;
            for tube in start..end - 1 {
                f.write_fmt(format_args!("|{}| ", self.0[tube].0[row]))?
            }
            f.write_fmt(format_args!("|{}|\n", self.0[end - 1].0[row]))?
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut puzzle = Puzzle::new(12);
    let mut rl = Editor::<()>::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let arr = line.split(" ").map(|s| s.trim()).collect::<Vec<&str>>();
                match arr[0] {
                    "i" | "init" => {
                        if let Some(size) = parse_int(arr.get(1)) {
                            puzzle = Puzzle::new(size);
                        } else {
                            eprintln!("usage: init <size>")
                        }
                    }
                    "s" | "set" => {
                        if let (Some(tube), Some(idx), Some(colour)) = (
                            parse_int(arr.get(1)),
                            parse_int(arr.get(2)),
                            arr.get(3)
                                .and_then(|s| <&&str as TryInto<Water>>::try_into(s).ok()),
                        ) {
                            puzzle.0[tube].0[idx] = State::Sticky(colour)
                        } else {
                            eprintln!("usage: set <tube> <idx> <colour>")
                        }
                    }
                    "p" | "print" => println!("{}", puzzle),
                    a => println!("Unrecognized command: {}", a),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt")
}

fn parse_int(i: Option<&&str>) -> Option<usize> {
    i.and_then(|s| s.parse::<usize>().ok())
}
