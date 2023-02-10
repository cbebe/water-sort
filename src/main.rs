use ansi_term::{Colour, Style};
use rustyline::{self, error::ReadlineError, Editor};
use serde::{Deserialize, Serialize};
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
        match *value {
            "a" | "ash" => Ok(Self::Ash),
            "b" | "bl" | "blue" => Ok(Self::Blue),
            "br" | "brown" => Ok(Self::Brown),
            "c" | "cy" | "cyan" => Ok(Self::Cyan),
            "g" | "green" => Ok(Self::Green),
            "l" | "lime" => Ok(Self::Lime),
            "ol" | "olive" => Ok(Self::Olive),
            "o" | "or" => Ok(Self::Orange),
            "p" | "pi" | "pink" => Ok(Self::Pink),
            "pu" | "purple" => Ok(Self::Purple),
            "r" | "red" => Ok(Self::Red),
            "y" | "yellow" => Ok(Self::Yellow),
            _ => Err(WaterError::UnknownColour),
        }
    }
}

impl Water {
    fn get_colour(&self) -> Colour {
        use Colour::RGB;
        match self {
            Self::Blue => RGB(58, 46, 195),
            Self::Brown => RGB(126, 74, 7),
            Self::Cyan => RGB(84, 163, 228),
            Self::Green => RGB(17, 101, 51),
            Self::Ash => RGB(99, 100, 101),
            Self::Lime => RGB(98, 214, 124),
            Self::Olive => RGB(120, 150, 15),
            Self::Orange => RGB(232, 140, 66),
            Self::Pink => RGB(234, 94, 123),
            Self::Purple => RGB(113, 43, 147),
            Self::Red => RGB(197, 42, 35),
            Self::Yellow => RGB(241, 217, 87),
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
struct Tube([State; 4]);

impl Tube {
    const fn empty() -> Self {
        Self([State::Empty, State::Empty, State::Empty, State::Empty])
    }

    #[allow(dead_code)]
    fn top(self) -> State {
        use State::Empty;
        match (self.0[0], self.0[1], self.0[2], self.0[3]) {
            (s, Empty, Empty, Empty) | (_, s, Empty, Empty) | (_, _, s, Empty) | (_, _, _, s) => s,
        }
    }

    #[allow(dead_code)]
    fn has_space(self) -> bool {
        matches!(self.0[3], State::Empty)
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Puzzle(Vec<Tube>);

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
    fn new(size: usize) -> Self {
        let mut tubes = vec![Tube::default(); size - 2];
        // The other two tubes will always be empty
        tubes.push(Tube::empty());
        tubes.push(Tube::empty());
        Self(tubes)
    }

    fn reset(&mut self, p: Self) {
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
                f.write_fmt(format_args!("|{}| ", self.0[tube].0[row]))?
            }
            f.write_fmt(format_args!("|{}|\n", self.0[end - 1].0[row]))?
        }
        Ok(())
    }
}

enum REPLError {
    Message(String),
    UnknownError,
}

impl From<&'static str> for REPLError {
    fn from(value: &'static str) -> Self {
        REPLError::Message(value.to_owned())
    }
}

fn process_command(puzzle: &mut Puzzle, command: &str, args: &[&str]) -> Result<(), REPLError> {
    match command {
        "i" | "init" => match parse_int(args.get(1)) {
            Some(size) if size > 2 => Ok(puzzle.reset(Puzzle::new(size))),
            Some(_) => Err(REPLError::from("size must be greater than 2")),
            None => Err(REPLError::from("usage: init <size>")),
        },
        "load" => {
            match args
                .get(0)
                // TODO: Handle errors
                .and_then(|f| fs::read_to_string(f).ok())
                .and_then(|s| serde_json::from_str::<Puzzle>(&s).ok())
            {
                Some(p) => {
                    puzzle.reset(p);
                    println!("{puzzle}");
                    Ok(())
                }
                None => Err(REPLError::from("error loading file")),
            }
        }
        "save" => {
            if let Ok(json) = serde_json::to_string(&puzzle) {
                // TODO: Handle errors
                if args.get(0).and_then(|s| fs::write(s, json).ok()).is_none() {
                    eprintln!("error saving puzzle");
                }
                Ok(())
            } else {
                eprintln!("error serializing puzzle");
                Ok(())
            }
        }
        "u" | "unset" => {
            let size = puzzle.0.len();
            match (parse_int(args.get(0)), parse_int(args.get(1))) {
                (Some(tube), Some(idx)) if tube < size && idx < 4 => {
                    puzzle.0[tube].0[idx] = State::Empty;
                    Ok(())
                }
                (Some(tube), _) if tube >= size => {
                    eprintln!("tube number must be between 0 and {}", size - 1);
                    Ok(())
                }
                (_, Some(idx)) if idx >= 4 => {
                    eprintln!("index must be between 0 and 3");
                    Ok(())
                }
                (_, _) => {
                    eprintln!("usage: unset <tube> <idx>");
                    Ok(())
                }
            }
        }
        "s" | "set" => {
            let size = puzzle.0.len();
            match (
                parse_int(args.get(0)),
                parse_int(args.get(1)),
                parse_water(args.get(2)),
            ) {
                (Some(tube), Some(idx), Ok(colour)) if tube < size && idx < 4 => {
                    puzzle.0[tube].0[idx] = State::Sticky(colour);
                    Ok(())
                }
                (Some(tube), _, _) if tube >= size => {
                    eprintln!("tube number must be between 0 and {}", size - 1);

                    Ok(())
                }
                (_, Some(idx), _) if idx >= 4 => {
                    eprintln!("index must be between 0 and 3");

                    Ok(())
                }
                (_, _, Err(WaterError::UnknownColour)) => {
                    eprintln!("unknown colour: {}", args[2]);

                    Ok(())
                }
                (_, _, _) => {
                    eprintln!("usage: set <tube> <idx> <colour>");
                    Ok(())
                }
            }
        }
        "p" | "print" => {
            println!("{puzzle}");
            Ok(())
        }
        a => {
            println!("Unrecognized command: {a}");
            Ok(())
        }
    }
}

fn process_line(puzzle: &mut Puzzle, line: String) -> Result<(), REPLError> {
    let arr = line
        .split(' ')
        .filter_map(|s| match s.trim() {
            s if !s.is_empty() => Some(s),
            _ => None,
        })
        .collect::<Vec<&str>>();
    if let Some(command) = arr.get(0) {
        process_command(puzzle, command, &arr.as_slice()[1..])
    } else {
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
                process_line(&mut puzzle, line);
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
                println!("Error: {err:?}");
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
        .and_then(<&&str as TryInto<Water>>::try_into)
}
