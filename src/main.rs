use repl::{Error, Usage};
use rustyline::{self, error::ReadlineError, Editor};
use water::Water;

mod puzzle;
mod repl;
mod state;
mod tube;
mod water;

fn process_command(puzzle: &mut puzzle::Puzzle, command: &str, args: &[&str]) -> Result<(), Error> {
    match command {
        "i" | "init" => {
            let size = parse_int(args.first()).ok_or(Error::Usage(Usage::Init))?;
            (size > 2)
                .then(|| puzzle.reset(puzzle::Puzzle::new(size)))
                .ok_or(Error::InvalidPuzzleSize)
        }
        "load" => {
            let file = args.first().ok_or(Error::Usage(Usage::Load))?;
            let json = std::fs::read_to_string(file)?;
            let loaded_puzzle = serde_json::from_str::<puzzle::Puzzle>(&json)?;
            puzzle.reset(loaded_puzzle);
            println!("{puzzle}");
            Ok(())
        }
        "save" => {
            let json = serde_json::to_string(&puzzle)?;
            let file = args.first().ok_or(Error::Usage(Usage::Save))?;
            std::fs::write(file, json)?;
            Ok(())
        }
        "tt" => {
            let size = puzzle.size();
            for i in args.iter() {
                let tube = i
                    .chars()
                    .next()
                    .and_then(|d| d.to_digit(10))
                    .and_then(|d| d.try_into().ok())
                    .ok_or(Error::Usage(Usage::QuickTube))?;
                if tube >= size {
                    return Err(Error::InvalidTube(size));
                }
                for (idx, j) in i.chars().skip(1).enumerate() {
                    let c = Water::try_from(j.to_string().as_str())
                        .map_err(|e| Error::from_water(e, Usage::QuickTube))?;
                    puzzle.set_tube(tube, idx, state::State::Water(c));
                }
            }
            Ok(())
        }
        "t" | "tube" => {
            let size = puzzle.size();
            for w in args.windows(2).step_by(2) {
                let [i, water]: [&str; 2] = w.try_into().map_err(|_| Error::Usage(Usage::Tube))?;
                let tube = i.parse::<usize>().map_err(|_| Error::Usage(Usage::Tube))?;
                if tube >= size {
                    return Err(Error::InvalidTube(size));
                }
                for (idx, colour) in water.split(',').enumerate() {
                    let col =
                        Water::try_from(colour).map_err(|e| Error::from_water(e, Usage::Tube))?;
                    puzzle.set_tube(tube, idx, state::State::Water(col));
                }
            }
            Ok(())
        }
        "o" | "pour" => {
            let size = puzzle.size();
            match (parse_int(args.first()), parse_int(args.get(1))) {
                (Some(a), Some(b)) if a < size && b < size => {
                    if puzzle.pour(a, b) {
                        Ok(())
                    } else {
                        Err(Error::InvalidPour(a, b))
                    }
                }
                (Some(tube), _) if tube >= size => Err(Error::InvalidTube(size)),
                (_, Some(tube)) if tube >= size => Err(Error::InvalidTube(size)),
                (_, Some(idx)) if idx >= 4 => Err(Error::InvalidIndex),
                (_, _) => Err(Error::Usage(Usage::Unset)),
            }
        }
        "u" | "unset" => set_tube(puzzle.size(), args, Usage::Unset, |tube, idx| {
            puzzle.set_tube(tube, idx, state::State::Empty);
        }),
        "s" | "set" => {
            let colour =
                Water::try_from(args.get(2)).map_err(|e| Error::from_water(e, Usage::Set))?;
            set_tube(puzzle.size(), args, Usage::Set, |tube, idx| {
                puzzle.set_tube(tube, idx, state::State::Water(colour));
            })
        }
        "p" | "print" => {
            println!("{puzzle}");
            Ok(())
        }
        a => Err(Error::UnrecognizedCommand(a.to_owned())),
    }
}

fn process_line(puzzle: &mut puzzle::Puzzle, line: &str) -> Result<(), Error> {
    let arr = line
        .split(' ')
        .filter_map(|s| match s.trim() {
            s if !s.is_empty() => Some(s),
            _ => None,
        })
        .collect::<Vec<&str>>();
    arr.first().map_or(Ok(()), |command| {
        process_command(puzzle, command, &arr.as_slice()[1..])
    })
}

fn main() -> rustyline::Result<()> {
    let histfile = std::env::var("WATER_HISTFILE").unwrap_or_else(|_| "history.txt".to_owned());
    let mut puzzle = puzzle::Puzzle::new(12);
    let mut rl = Editor::<()>::new()?;
    if rl.load_history(&histfile).is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if let Err(e) = process_line(&mut puzzle, &line) {
                    eprintln!("{e}");
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
                println!("Error: {err:?}");
                break;
            }
        }
    }
    rl.save_history(&histfile)
}

fn parse_int(i: Option<&&str>) -> Option<usize> {
    i.and_then(|s| s.parse::<usize>().ok())
}

fn set_tube<F>(size: usize, args: &[&str], usage: repl::Usage, mut cb: F) -> Result<(), Error>
where
    F: FnMut(usize, usize),
{
    match (parse_int(args.first()), parse_int(args.get(1))) {
        (Some(tube), Some(idx)) if tube < size && idx < 4 => {
            cb(tube, idx);
            Ok(())
        }
        (Some(tube), _) if tube >= size => Err(Error::InvalidTube(size)),
        (_, Some(idx)) if idx >= 4 => Err(Error::InvalidIndex),
        (_, _) => Err(Error::Usage(usage)),
    }
}
