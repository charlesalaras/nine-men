use crate::CUTAWAYS;
use crate::LINE_SIZE;
use crate::OPERATORS;

use crate::node;

pub struct Problem {
    pub initial_state: node::Node,
    pub operators: [fn(node::Node, usize, usize) -> node::Node; OPERATORS],
}

impl Problem {
    pub fn init(initial_state: node::Node) -> Problem {
        Problem {
            initial_state,
            operators: [
                Problem::operator_up,
                Problem::operator_down,
                Problem::operator_left,
                Problem::operator_right,
            ],
        }
    }
    pub fn goal_test(state: [u32; CUTAWAYS + LINE_SIZE]) -> bool {
        let mut curr: u32 = 1;
        for i in 1..LINE_SIZE - 1 {
            if state[i - 1] != curr {
                return false;
            }
            curr += 1;
        }
        for i in LINE_SIZE..LINE_SIZE + CUTAWAYS {
            if state[i] != 0 {
                return false;
            }
        }
        true
    }
    fn operator_up(node: node::Node, i: usize, j: usize) -> node::Node {
        let mut state = node.state;
        state.swap(j, i);
        let new_node = node::Node::init(state, node.cutaways, node.g + 1, None);
        new_node
    }
    fn operator_down(node: node::Node, i: usize, j: usize) -> node::Node {
        let mut state = node.state;
        state.swap(j, i);
        let new_node = node::Node::init(state, node.cutaways, node.g + 1, None);
        new_node
    }
    fn operator_left(node: node::Node, i: usize, j: usize) -> node::Node {
        let mut state = node.state;
        state.swap(j, i);
        let new_node = node::Node::init(state, node.cutaways, node.g + 1, None);
        new_node
    }
    fn operator_right(node: node::Node, i: usize, j: usize) -> node::Node {
        let mut state = node.state;
        state.swap(j, i);
        let new_node = node::Node::init(state, node.cutaways, node.g + 1, None);
        new_node
    }
}
