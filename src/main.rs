use ansi_term::{Colour, Style};
use rustyline::{self, error::ReadlineError, Editor};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fmt::Display, fs};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
enum Water {
    Ash,
    Blue,
    Brown,
    Cyan,
    Green,
    Lime,
    Olive,
    Orange,
    Pink,
    Purple,
    Red,
    Yellow,
}

enum WaterError {
    Empty,
    UnknownColour,
}

impl TryFrom<&&str> for Water {
    type Error = WaterError;

    fn try_from(value: &&str) -> std::result::Result<Self, Self::Error> {
        use Water::*;
        match *value {
            "a" | "ash" => Ok(Ash),
            "b" | "bl" | "blue" => Ok(Blue),
            "br" | "brown" => Ok(Brown),
            "c" | "cy" | "cyan" => Ok(Cyan),
            "g" | "green" => Ok(Green),
            "l" | "lime" => Ok(Lime),
            "ol" | "olive" => Ok(Olive),
            "o" | "or" => Ok(Orange),
            "p" | "pi" | "pink" => Ok(Pink),
            "pu" | "purple" => Ok(Purple),
            "r" | "red" => Ok(Red),
            "y" | "yellow" => Ok(Yellow),
            _ => Err(WaterError::UnknownColour),
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
            Water::Ash => RGB(99, 100, 101),
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

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
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

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
struct Tube([State; 4]);

impl Tube {
    const fn empty() -> Self {
        Self([State::Empty, State::Empty, State::Empty, State::Empty])
    }

    #[allow(dead_code)]
    fn top(&self) -> State {
        use State::Empty;
        match (self.0[0], self.0[1], self.0[2], self.0[3]) {
            (s, Empty, Empty, Empty) => s,
            (_, s, Empty, Empty) => s,
            (_, _, s, Empty) => s,
            (_, _, _, s) => s,
        }
    }

    #[allow(dead_code)]
    fn has_space(&self) -> bool {
        matches!(self.0[3], State::Empty)
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
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

fn main() -> rustyline::Result<()> {
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
                let arr = line.split(' ').map(|s| s.trim()).collect::<Vec<&str>>();
                match arr[0] {
                    "i" | "init" => match parse_int(arr.get(1)) {
                        Some(size) if size > 2 => puzzle = Puzzle::new(size),
                        Some(_) => eprintln!("size must be greater than 2"),
                        None => eprintln!("usage: init <size>"),
                    },
                    "load" => {
                        match arr
                            .get(1)
                            // TODO: Handle errors
                            .and_then(|f| fs::read_to_string(f).ok())
                            .and_then(|s| serde_json::from_str::<Puzzle>(&s).ok())
                        {
                            Some(p) => {
                                puzzle = p;
                                println!("{}", puzzle);
                            }
                            None => eprintln!("error loading file"),
                        }
                    }
                    "save" => {
                        if let Ok(json) = serde_json::to_string(&puzzle) {
                            // TODO: Handle errors
                            if let None = arr.get(1).and_then(|s| fs::write(s, json).ok()) {
                                eprintln!("error saving puzzle");
                            }
                        } else {
                            eprintln!("error serializing puzzle");
                        }
                    }
                    "u" | "unset" => {
                        let size = puzzle.0.len();
                        match (parse_int(arr.get(1)), parse_int(arr.get(2))) {
                            (Some(tube), Some(idx)) if tube < size && idx < 4 => {
                                puzzle.0[tube].0[idx] = State::Empty;
                            }
                            (Some(tube), _) if tube >= size => {
                                eprintln!("tube number must be between 0 and {}", size - 1);
                            }
                            (_, Some(idx)) if idx >= 4 => {
                                eprintln!("index must be between 0 and 3");
                            }
                            (_, _) => {
                                eprintln!("usage: unset <tube> <idx>");
                            }
                        }
                    }
                    "s" | "set" => {
                        let size = puzzle.0.len();
                        match (
                            parse_int(arr.get(1)),
                            parse_int(arr.get(2)),
                            parse_water(arr.get(3)),
                        ) {
                            (Some(tube), Some(idx), Ok(colour)) if tube < size && idx < 4 => {
                                puzzle.0[tube].0[idx] = State::Sticky(colour);
                            }
                            (Some(tube), _, _) if tube >= size => {
                                eprintln!("tube number must be between 0 and {}", size - 1);
                            }
                            (_, Some(idx), _) if idx >= 4 => {
                                eprintln!("index must be between 0 and 3");
                            }
                            (_, _, Err(WaterError::UnknownColour)) => {
                                eprintln!("unknown colour: {}", arr[3]);
                            }
                            (_, _, _) => {
                                eprintln!("usage: set <tube> <idx> <colour>");
                            }
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

fn parse_water(w: Option<&&str>) -> std::result::Result<Water, WaterError> {
    w.ok_or(WaterError::Empty)
        .and_then(|s| <&&str as TryInto<Water>>::try_into(s))
}
