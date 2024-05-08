use crate::node;
use crate::problem;
use crate::runtime;
use crate::runtime::Algorithm;

use colored::Colorize;
use std::cmp;
use std::collections::BinaryHeap;

use crate::CUTAWAYS;
use crate::LINE_SIZE;
use crate::OPERATORS;

fn expand(
    node: &node::Node,
    operators: [fn(&node::Node, usize, usize, Algorithm) -> node::Node; OPERATORS],
    runtime: &mut runtime::Runtime,
) -> Vec<node::Node> {
    runtime.print(format!(
        "The best state to expand with a {} = {} and {} = {} is:\n{}\n",
        "g(n)".green(),
        node.g,
        "h(n)".blue(),
        node.h,
        node.print()
    ));
    let mut new_nodes: Vec<node::Node> = Vec::<node::Node>::new();
    // Check for duplicates
    if runtime.seen.contains(&node) {
        return new_nodes;
    }

    runtime.seen.insert(*node);
    runtime.nodes_expanded = runtime.nodes_expanded + 1;
    for i in &node.zero_tiles {
        // Check if its a cutaway tile
        for j in 0..CUTAWAYS {
            // A zero exists in the line which can be swapped with the cutaway
            if *i == node.cutaways[j] && node.state[LINE_SIZE + j] != 0 {
                new_nodes.push(operators[0](node, *i, LINE_SIZE + j, runtime.search));
            }
            // A zero exists in the cutaway which can be swapped with the line
            if *i == LINE_SIZE + j && node.state[node.cutaways[j]] != 0 {
                new_nodes.push(operators[0](node, *i, node.cutaways[j], runtime.search));
            }
        }
        if *i < LINE_SIZE {
            if *i != LINE_SIZE - 1 {
                if node.state[i + 1] != 0 {
                    new_nodes.push(operators[0](node, *i, *i + 1, runtime.search));
                }
            }
            if *i != 0 {
                if node.state[i - 1] != 0 {
                    new_nodes.push(operators[0](node, *i, *i - 1, runtime.search));
                }
            }
        }
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
    runtime: &mut runtime::Runtime,
) -> Option<node::Node> {
    //let mut i = 0;
    let mut nodes = BinaryHeap::<node::Node>::new();
    nodes.push(problem.initial_state);
    while !nodes.is_empty() {
        //while i < 20 {
        runtime.max_size = cmp::max(runtime.max_size, nodes.len());
        let node = nodes.pop().unwrap();
        if problem::Problem::goal_test(node.state) {
            return Some(node);
        }
        // insert into nodes by recreating heap using queueing function
        nodes = queueing_function(nodes, expand(&node, problem.operators, runtime));
        //i = i + 1;
    }
    None
}
