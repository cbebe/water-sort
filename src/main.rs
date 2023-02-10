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
        "t" | "tube" => {
            let size = puzzle.size();
            for w in args.windows(2).step_by(2) {
                let [i, water]: [&str; 2] = w.try_into().unwrap();
                let tube = i.parse::<usize>().map_err(|_| Error::Usage(Usage::Tube))?;
                if tube >= size {
                    return Err(Error::InvalidTubeNumber(size));
                }
                for (idx, colour) in water.split(',').enumerate() {
                    match Water::try_from(colour) {
                        Ok(col) => {
                            puzzle.set_tube(tube, idx, state::State::Water(col));
                            Ok(())
                        }
                        Err(water::ParseError::Empty) => Err(Error::Usage(Usage::Tube)),
                        Err(water::ParseError::UnknownColour) => {
                            Err(Error::UnknownWaterColour(colour.to_owned()))
                        }
                    }?;
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
                (Some(tube), _) if tube >= size => Err(Error::InvalidTubeNumber(size)),
                (_, Some(tube)) if tube >= size => Err(Error::InvalidTubeNumber(size)),
                (_, Some(idx)) if idx >= 4 => Err(Error::InvalidIndex),
                (_, _) => Err(Error::Usage(Usage::Unset)),
            }
        }
        "u" | "unset" => {
            let size = puzzle.size();
            match (parse_int(args.first()), parse_int(args.get(1))) {
                (Some(tube), Some(idx)) if tube < size && idx < 4 => {
                    puzzle.set_tube(tube, idx, state::State::Empty);
                    Ok(())
                }
                (Some(tube), _) if tube >= size => Err(Error::InvalidTubeNumber(size)),
                (_, Some(idx)) if idx >= 4 => Err(Error::InvalidIndex),
                (_, _) => Err(Error::Usage(Usage::Unset)),
            }
        }
        "s" | "set" => {
            let size = puzzle.size();
            match (
                parse_int(args.first()),
                parse_int(args.get(1)),
                Water::try_from(args.get(2)),
            ) {
                (Some(tube), Some(idx), Ok(colour)) if tube < size && idx < 4 => {
                    puzzle.set_tube(tube, idx, state::State::Water(colour));
                    Ok(())
                }
                (Some(tube), _, _) if tube >= size => Err(Error::InvalidTubeNumber(size)),
                (_, Some(idx), _) if idx >= 4 => Err(Error::InvalidIndex),
                (_, _, Err(water::ParseError::UnknownColour)) => {
                    Err(Error::UnknownWaterColour(args[2].to_owned()))
                }
                (_, _, _) => Err(Error::Usage(Usage::Set)),
            }
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
    let mut puzzle = puzzle::Puzzle::new(12);
    let mut rl = Editor::<()>::new()?;
    if rl.load_history("history.txt").is_err() {
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
    rl.save_history("history.txt")
}

fn parse_int(i: Option<&&str>) -> Option<usize> {
    i.and_then(|s| s.parse::<usize>().ok())
}
