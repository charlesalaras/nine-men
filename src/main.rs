mod node;
mod problem;

const LINE_SIZE: usize = 10;
const CUTAWAYS: usize = 3;

use std::collections::BinaryHeap;

fn expand(node: node::Node, operators: problem::Problem) {}

fn queueing_function(nodes: BinaryHeap<i32>) -> BinaryHeap<i32> {
    nodes
}

fn search(prob: problem::Problem) -> Option<i32> {
    let mut nodes = BinaryHeap::new();
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
    println!("Hello world!");
}
