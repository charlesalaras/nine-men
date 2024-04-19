mod node;
mod problem;

enum Algorithm {
    UniformCost,
    MisplacedTile,
    ManhattanDist,
}

/*
Parameters for problem:
- LINE_SIZE: Specifies the length of the line
- CUTAWAYS: Specifies how many recesses exist in the line that can be moved into
- SEARCH_ALGO: Specifies the heuristic to use (see above enum Algorithm)
- GOAL_STATE: Specifies what the final solution should look like*
  * Note that this is dependent on the initial state, so not every initial state
  has the same goal state.
*/
const LINE_SIZE: usize = 10;
const CUTAWAYS: usize = 3;
static SEARCH_ALGO: Algorithm = Algorithm::UniformCost;
static GOAL_STATE: [u32; LINE_SIZE + CUTAWAYS] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0];

use std::collections::BinaryHeap;

// Returns a list of nodes that are expanded from node using operators.
fn expand(node: node::Node, operators: problem::Problem) {}

// Updates the nodes queue with the provided list of nodes.
fn queueing_function(nodes: &mut BinaryHeap<node::Node>) -> &mut BinaryHeap<node::Node> {
    nodes
}

/*
Generic search algorithm altered with heuristics through usage of queueing_function

Returns an Option where:
- Some() value means a node with the solution was found
- None value which means no node could be found
*/
fn search(problem: problem::Problem) -> Option<node::Node> {
    let mut nodes = BinaryHeap::<node::Node>::new();
    nodes.push(problem.initial_state);
    while !nodes.is_empty() {
        let node = nodes.pop().unwrap();
        if problem::Problem::goal_test(node.state) {
            return Some(node);
        }
        // insert into nodes by recreating heap using queueing function
        nodes = queueing_function(&mut nodes);
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
