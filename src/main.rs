use rustyline::{self, error::ReadlineError, Editor};

use repl::{Error, Usage};
use solve::{dfs_puzzle, NoSolution};
use water::Water;

mod puzzle;
mod repl;
mod solve;
mod state;
mod tube;
mod water;

fn load_file(puzzle: &mut puzzle::Puzzle, args: &[&str]) -> Result<(), Error> {
    let file = args.first().ok_or(Error::Usage(Usage::Load))?;
    let json = std::fs::read_to_string(file)?;
    let loaded_puzzle = serde_json::from_str::<puzzle::Puzzle>(&json)?;
    puzzle.reset(loaded_puzzle);
    Ok(println!("{puzzle}"))
}

fn quick_tube(puzzle: &mut puzzle::Puzzle, args: &[&str]) -> Result<(), Error> {
    let size = puzzle.size();
    for i in args.iter() {
        let tube = i
            .chars()
            .next()
            .and_then(|d| d.to_digit(16))
            .and_then(|d| d.try_into().ok())
            .ok_or(Error::Usage(Usage::QuickTube))?;
        if tube >= size {
            return Err(Error::InvalidTube(tube, size));
        }
        for (idx, j) in i.chars().skip(1).enumerate() {
            let c = Water::try_from(j.to_string().as_str())
                .map_err(|e| Error::from_water(e, Usage::QuickTube))?;
            puzzle.set_tube(tube, idx, state::State::Water(c));
        }
    }
    Ok(())
}

fn process_command(puzzle: &mut puzzle::Puzzle, command: &str, args: &[&str]) -> Result<(), Error> {
    match command {
        "i" | "init" => {
            let size = parse_int(args.first()).ok_or(Error::Usage(Usage::Init))?;
            (size > 2)
                .then(|| puzzle.reset(puzzle::Puzzle::new(size)))
                .ok_or(Error::InvalidPuzzleSize)
        }
        "load" => load_file(puzzle, args),
        "solve" => Ok(match dfs_puzzle(puzzle) {
            Ok(solution) => println!("{solution}"),
            Err(NoSolution::AlreadySolved) => println!("already solved"),
            Err(NoSolution::CannotBeSolved(moves, max_depth)) => {
                println!("cannot be solved... max depth: {max_depth}");
                println!("{moves}")
            }
        }),
        "save" => Ok(std::fs::write(
            args.first().ok_or(Error::Usage(Usage::Save))?,
            serde_json::to_string(puzzle)?,
        )?),
        "tt" => quick_tube(puzzle, args),
        "t" | "tube" => {
            let size = puzzle.size();
            for w in args.windows(2).step_by(2) {
                let [i, water]: [&str; 2] = w.try_into().or(Err(Error::Usage(Usage::Tube)))?;
                let tube = i.parse::<usize>().or(Err(Error::Usage(Usage::Tube)))?;
                if tube >= size {
                    return Err(Error::InvalidTube(tube, size));
                }
                for (idx, colour) in water.split(',').enumerate() {
                    let col =
                        Water::try_from(colour).map_err(|e| Error::from_water(e, Usage::Tube))?;
                    puzzle.set_tube(tube, idx, state::State::Water(col));
                }
            }
            Ok(())
        }
        "p" | "pour" => {
            let size = puzzle.size();
            match (parse_int(args.first()), parse_int(args.get(1))) {
                (Some(a), Some(b)) if a < size && b < size => puzzle
                    .pour(a, b)
                    .map_err(|e| Error::InvalidPour(e.from, e.to)),
                (Some(tube), _) if tube >= size => Err(Error::InvalidTube(tube, size)),
                (_, Some(tube)) if tube >= size => Err(Error::InvalidTube(tube, size)),
                (_, Some(idx)) if idx >= 4 => Err(Error::InvalidIndex),
                (_, _) => Err(Error::Usage(Usage::Pour)),
            }
        }
        "u" | "unset" => set_tube(puzzle.size(), args, Usage::Unset, |tube, idx| {
            puzzle.set_tube(tube, idx, state::State::Unknown);
        }),
        "e" | "empty" => set_tube(puzzle.size(), args, Usage::Empty, |tube, idx| {
            puzzle.set_tube(tube, idx, state::State::Empty)
        }),
        "s" | "set" => {
            let colour =
                Water::try_from(args.get(2)).map_err(|e| Error::from_water(e, Usage::Set))?;
            set_tube(puzzle.size(), args, Usage::Set, |tube, idx| {
                puzzle.set_tube(tube, idx, state::State::Water(colour));
            })
        }
        "d" | "display" => Ok(println!("{puzzle}")),
        "v" | "valid" => Ok(println!("{}", puzzle.valid_moves())),
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
        (Some(tube), _) if tube >= size => Err(Error::InvalidTube(tube, size)),
        (_, Some(idx)) if idx >= 4 => Err(Error::InvalidIndex),
        (_, _) => Err(Error::Usage(usage)),
    }
}
