use crate::node;
use crate::problem;
use crate::runtime;

use std::cmp;
use std::collections::BinaryHeap;

use crate::CUTAWAYS;
use crate::LINE_SIZE;
use crate::OPERATORS;

fn expand(
    node: node::Node,
    operators: [fn(node::Node, usize, usize) -> node::Node; OPERATORS],
    runtime: &mut runtime::Runtime,
) -> Vec<node::Node> {
    runtime.print(format!(
        "The best state to expand with a g(n) = {} and h(n) = {} is:\n{}\n",
        node.g,
        node.h,
        node.print()
    ));
    runtime.nodes_expanded = runtime.nodes_expanded + 1;
    let mut new_nodes: Vec<node::Node> = Vec::<node::Node>::new();

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
    problem: &problem::Problem,
    queueing_function: fn(BinaryHeap<node::Node>, Vec<node::Node>) -> BinaryHeap<node::Node>,
    runtime: &mut runtime::Runtime,
) -> Option<node::Node> {
    let mut nodes = BinaryHeap::<node::Node>::new();
    nodes.push(problem.initial_state);
    while !nodes.is_empty() {
        runtime.max_size = cmp::max(runtime.max_size, nodes.len());
        let node = nodes.pop().unwrap();
        if problem::Problem::goal_test(node.state) {
            return Some(node);
        }
        // insert into nodes by recreating heap using queueing function
        nodes = queueing_function(nodes, expand(node, problem.operators, runtime));
    }
    None
}
