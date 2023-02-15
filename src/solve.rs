use crate::puzzle::{Puzzle, ValidMoves};
use indextree_ng::{Arena, NodeId};
use std::collections::{HashSet, VecDeque};

pub struct PuzzleState {
    pub puzzle: Puzzle,
    pub played_move: (u8, u8),
    depth: usize,
}

pub enum NoSolution {
    AlreadySolved,
    CannotBeSolved(ValidMoves, usize),
    HasUnknown(Puzzle, ValidMoves),
}

pub fn dfs_puzzle(p: &Puzzle) -> Result<ValidMoves, NoSolution> {
    if p.is_solved() {
        return Err(NoSolution::AlreadySolved);
    }
    if p.has_unknown() {
        return Err(NoSolution::HasUnknown(p.clone(), ValidMoves(vec![])));
    }

    let mut arena: Arena<PuzzleState> = Arena::new();
    let mut stack: VecDeque<NodeId> = VecDeque::new();

    for m in &p.valid_moves().get() {
        let mut new_p = p.clone();
        new_p.pour(m.0.into(), m.1.into()).unwrap();
        let node = arena.new_node(PuzzleState {
            puzzle: new_p.clone(),
            played_move: *m,
            depth: 0,
        });

        stack.push_back(node);
    }

    let mut max_depth = 0;
    let mut max_moves = ValidMoves(vec![]);

    let mut visited = HashSet::<Puzzle>::new();

    while let Some(id) = stack.pop_back() {
        let node = &arena[id];
        let puzzle = node.data.puzzle.clone();
        if puzzle.is_solved() {
            return Ok(get_move_chain(&arena, id));
        }
        if p.has_unknown() {
            return Err(NoSolution::HasUnknown(
                p.clone(),
                get_move_chain(&arena, id),
            ));
        }
        if visited.contains(&puzzle) {
            continue;
        }
        visited.insert(puzzle.clone());

        let moves = puzzle.valid_moves().get();
        if moves.is_empty() {
            if node.data.depth > max_depth {
                max_moves = get_move_chain(&arena, id);
                max_depth = node.data.depth;
                dbg!(max_depth);
            }
            arena.remove_node(id);
            continue;
        }

        for m in &moves {
            let mut new_p = puzzle.clone();
            new_p.pour(m.0.into(), m.1.into()).unwrap();
            let child_id = arena.new_node(PuzzleState {
                puzzle: new_p.clone(),
                played_move: *m,
                depth: arena[id].data.depth + 1,
            });
            match id.append(child_id, &mut arena) {
                Ok(_) => stack.push_back(child_id),
                Err(err) => {
                    dbg!(err);
                }
            }
        }
    }

    Err(NoSolution::CannotBeSolved(max_moves, max_depth))
}

fn get_move_chain(arena: &Arena<PuzzleState>, id: NodeId) -> ValidMoves {
    let mut moves: Vec<(u8, u8)> = id
        .ancestors(arena)
        .into_iter()
        .map(|d| arena[d].data.played_move)
        .collect();
    moves.reverse();
    ValidMoves(moves)
}

#[cfg(test)]
mod solve_test {
    use crate::puzzle::Puzzle;
    use crate::solve::dfs_puzzle;
    use crate::state::State::Water;
    use crate::water::Water::{Blue, Green, Red};

    #[test]
    fn test_solve() {
        let mut p = Puzzle::new(5);
        p.set_whole_tube(0, [Water(Green), Water(Blue), Water(Red), Water(Green)]);
        p.set_whole_tube(1, [Water(Blue), Water(Blue), Water(Red), Water(Green)]);
        p.set_whole_tube(2, [Water(Blue), Water(Red), Water(Red), Water(Green)]);

        assert!(matches!(dfs_puzzle(&p), Ok(_)));
    }
}
