mod node;
mod problem;

enum Algorithm {
    UniformCost,
    MisplacedTile,
    ManhattanDist,
}

const LINE_SIZE: usize = 10;
const CUTAWAYS: usize = 3;
static SEARCH_ALGO: Algorithm = Algorithm::UniformCost;

use std::collections::BinaryHeap;

fn expand(node: node::Node, operators: problem::Problem) {}

fn queueing_function(nodes: &mut BinaryHeap<node::Node>) -> &mut BinaryHeap<node::Node> {
    nodes
}

fn search(prob: problem::Problem) -> Option<i32> {
    let mut nodes = BinaryHeap::<node::Node>::new();
    nodes.push(1);
    while !nodes.is_empty() {
        let node = nodes.pop().unwrap();
        if true {
            return Some(node);
        }
        // insert into nodes by recreating heap using queueing function
    }
    None
}

fn main() {
    let mut problem: problem::Problem = problem::Problem {
        initial_state: node::Node {
            state: [0, 2, 3, 4, 5, 6, 7, 8, 9, 1, 0, 0, 0],
            cutaways: [3, 5, 7],
            zero_tile: 0,
            g: 0,
            h: 0,
        },
        operators: [operator_swap, operator_cutaway()],
    };
}
