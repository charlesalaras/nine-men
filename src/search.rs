use crate::node;
use crate::problem;
use crate::runtime;

use std::collections::BinaryHeap;

use crate::CUTAWAYS;
use crate::LINE_SIZE;
use crate::OPERATORS;

fn expand(
    node: node::Node,
    operators: [fn(node::Node, usize, usize) -> node::Node; OPERATORS],
) -> Vec<node::Node> {
    let mut new_nodes: Vec<node::Node> = Vec::<node::Node>::new();
    // Check the cutaways
    for i in 0..CUTAWAYS {
        // Move Tile Up
        if node.state[node.cutaways[i]] != 0 && node.state[LINE_SIZE + i] == 0 {
            new_nodes.push(operators[0](node, LINE_SIZE + 1, node.cutaways[i]));
        }
        // Move Tile Down
        if node.state[node.cutaways[i]] == 0 && node.state[LINE_SIZE + i] != 0 {
            new_nodes.push(operators[1](node, node.cutaways[i], LINE_SIZE + 1));
        }
    }
    // Move Tile Right
    if node.zero_tile == 0 {
        new_nodes.push(operators[2](node, node.zero_tile, 1));
    }
    // Move Tile Left
    else if node.zero_tile == LINE_SIZE - 1 {
        new_nodes.push(operators[3](node, LINE_SIZE - 1, node.zero_tile));
    }
    // Move Tile Left and Right
    else {
        new_nodes.push(operators[2](node, node.zero_tile, node.zero_tile - 1));
        new_nodes.push(operators[3](node, node.zero_tile, node.zero_tile + 1));
    }
    new_nodes
}

// Updates the nodes queue with the provided list of nodes.
pub fn queueing_function(
    mut nodes: BinaryHeap<node::Node>,
    new_nodes: Vec<node::Node>,
) -> BinaryHeap<node::Node> {
    for i in new_nodes {
        nodes.push(i);
    }
    nodes
}

/*
Generic search algorithm altered with heuristics through usage of queueing_function

Returns an Option where:
- Some() value means a node with the solution was found
- None value which means no node could be found
*/
pub fn search(
    problem: problem::Problem,
    queueing_function: fn(BinaryHeap<node::Node>, Vec<node::Node>) -> BinaryHeap<node::Node>,
) -> Option<node::Node> {
    let mut nodes = BinaryHeap::<node::Node>::new();
    nodes.push(problem.initial_state);
    while !nodes.is_empty() {
        let node = nodes.pop().unwrap();
        if problem::Problem::goal_test(node.state) {
            return Some(node);
        }
        // insert into nodes by recreating heap using queueing function
        nodes = queueing_function(nodes, expand(node, problem.operators));
    }
    None
}
