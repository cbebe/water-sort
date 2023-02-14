use crate::puzzle::{Puzzle, ValidMoves};
use std::collections::VecDeque;

pub struct PuzzleState(pub Puzzle, pub (usize, usize));

pub fn solve_puzzle(p: &Puzzle) -> Option<ValidMoves> {
    if p.is_solved() {
        return None;
    }

    let mut arena: Arena<PuzzleState> = Arena { nodes: vec![] };
    let mut stack: VecDeque<NodeId> = VecDeque::new();

    for m in p.valid_moves().to_owned().iter() {
        let mut new_p = p.clone();
        new_p.pour(m.0, m.1);
        let id = arena.new_node(PuzzleState(new_p.to_owned(), m.clone()), None);

        stack.push_back(id);
    }

    // let mut i = 0;
    while let Some(id) = stack.pop_back() {
        let puzzle = dbg!(arena.get(id).data.0.clone());
        if puzzle.is_solved() {
            let mut curr = id;

            let mut moves: Vec<(usize, usize)> = vec![arena.get(curr).data.1];

            while let Some(parent_id) = arena.get(curr).parent {
                let node = arena.get(parent_id);
                moves.push(node.data.1);
                curr = parent_id;
            }
            moves.reverse();
            return Some(ValidMoves(moves));
        }

        let moves = dbg!(puzzle.valid_moves().to_owned());
        // i += 1;
        // if i == 10 {
        //     panic!("done");
        // }
        if moves.is_empty() {
            arena.remove(id);
            continue;
        }

        for m in moves.iter() {
            let mut new_p = puzzle.clone();
            new_p.pour(m.0, m.1);
            let child_id = arena.new_node(PuzzleState(new_p.to_owned(), m.clone()), Some(id));
            stack.push_back(child_id);
        }
    }

    None
}

// Arena {{{

// https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/
pub struct Arena<T> {
    pub nodes: Vec<Node<T>>,
}

pub struct Node<T> {
    pub parent: Option<NodeId>,
    pub data: T,
}

#[derive(Debug, Clone, Copy)]
pub struct NodeId {
    index: usize,
}

impl<T> Arena<T> {
    pub fn new_node(&mut self, data: T, parent: Option<NodeId>) -> NodeId {
        let next_index = self.nodes.len();
        self.nodes.push(Node { parent, data });
        NodeId { index: next_index }
    }

    pub fn get(&self, id: NodeId) -> &Node<T> {
        &self.nodes[id.index]
    }

    pub fn remove(&mut self, id: NodeId) {
        self.nodes.remove(id.index);
    }
}
// }}}

#[cfg(test)]
mod solve_test {
    use crate::puzzle::Puzzle;
    use crate::solve::solve_puzzle;
    use crate::state::State::Water;
    use crate::water::Water::{Blue, Green, Red};

    #[ignore]
    #[test]
    fn test_solve() {
        let mut p = Puzzle::new(5);
        p.set_whole_tube(0, [Water(Green), Water(Blue), Water(Red), Water(Green)]);
        p.set_whole_tube(1, [Water(Blue), Water(Blue), Water(Red), Water(Green)]);
        p.set_whole_tube(2, [Water(Blue), Water(Red), Water(Red), Water(Green)]);

        if let Some(v) = solve_puzzle(&p) {
            dbg!(v.to_owned());
        } else {
            panic!("did not find a solution")
        }
    }
}
