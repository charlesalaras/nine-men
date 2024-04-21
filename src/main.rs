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
- GOAL_STATE: Specifies what the final solution should look like
*/
const LINE_SIZE: usize = 10;
const CUTAWAYS: usize = 3;
static SEARCH_ALGO: Algorithm = Algorithm::UniformCost;
static GOAL_STATE: [u32; LINE_SIZE + CUTAWAYS] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0];

//use std::collections::BinaryHeap;
/*
// Returns a list of nodes that are expanded from node using operators.
fn expand(_node: node::Node, _operators: problem::Problem) -> Vec<node::Node> {
    let new_nodes: Vec<node::Node> = Vec::<node::Node>::new();

    new_nodes
}

// Updates the nodes queue with the provided list of nodes.
fn queueing_function(
    nodes: &mut BinaryHeap<node::Node>,
    new_nodes: Vec<node::Node>,
) -> BinaryHeap<node::Node> {
    for i in new_nodes {
        nodes.push(i);
    }
    *nodes
}

/*
Generic search algorithm altered with heuristics through usage of queueing_function

Returns an Option where:
- Some() value means a node with the solution was found
- None value which means no node could be found
*/
fn search(
    problem: problem::Problem,
    queueing_function: fn(&mut BinaryHeap<node::Node>, Vec<node::Node>) -> BinaryHeap<node::Node>,
) -> Option<node::Node> {
    let mut nodes = BinaryHeap::<node::Node>::new();
    nodes.push(problem.initial_state);
    while !nodes.is_empty() {
        let node = nodes.pop().unwrap();
        if problem::Problem::goal_test(node.state) {
            return Some(node);
        }
        // insert into nodes by recreating heap using queueing function
        nodes = queueing_function(&mut nodes, expand(node, problem));
    }
    None
}
*/
fn main() {
    /*
    let mut problem: problem::Problem = problem::Problem {
        initial_state: node::Node {
            state: [0, 2, 3, 4, 5, 6, 7, 8, 9, 1, 0, 0, 0],
            cutaways: [3, 5, 7],
            zero_tile: 0,
            g: 0,
            h: 0,
        },
        operators: [problem::operator_swap, operator_cutaway()],
    };
    */
    let node: node::Node = node::Node::init(
        [0, 2, 3, 4, 5, 6, 7, 8, 9, 1, 0, 0, 0],
        [3, 5, 7],
        0,
        Some(0),
    );
    println!("Hello world!");
    node.print();
}
