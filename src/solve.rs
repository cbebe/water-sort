use crate::puzzle::{Puzzle, ValidMoves};

struct PuzzleState(Puzzle, (usize, usize));

pub fn solve_puzzle(p: &Puzzle) -> Option<ValidMoves> {
    if p.is_solved() {
        return None;
    }

    let mut arena: Arena<PuzzleState> = Arena { nodes: vec![] };

    for m in p.valid_moves().to_owned().iter() {
        let mut new_p = p.clone();
        new_p.pour(m.0, m.1);
        let id = arena.new_node(PuzzleState(new_p.to_owned(), m.clone()), None);

        if let Some(id) = explore(&mut arena, id) {
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
    }
    None
}

fn explore(arena: &mut Arena<PuzzleState>, id: NodeId) -> Option<NodeId> {
    let puzzle = arena.get(id).data.0.clone();
    if puzzle.is_solved() {
        return Some(id);
    }
    let moves = puzzle.valid_moves().to_owned();
    if moves.is_empty() {
        // this is dfs, so it should be safe i guess
        arena.nodes.remove(id.index);
        return None;
    }

    for m in moves.iter() {
        let mut new_p = puzzle.clone();
        new_p.pour(m.0, m.1);
        let id = arena.new_node(PuzzleState(new_p.to_owned(), m.clone()), Some(id));
        if let Some(id) = explore(arena, id) {
            return Some(id);
        }
    }
    None
}

// Arena {{{

// https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/
pub struct Arena<T> {
    nodes: Vec<Node<T>>,
}

pub struct Node<T> {
    parent: Option<NodeId>,
    pub data: T,
}

#[derive(Clone, Copy)]
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
}
// }}}
