use crate::CUTAWAYS;
use crate::LINE_SIZE;
use crate::OPERATORS;

use crate::node;
use crate::runtime::Algorithm;

pub struct Problem {
    pub initial_state: node::Node,
    pub operators: [fn(&node::Node, usize, usize, Algorithm) -> node::Node; OPERATORS],
}

impl Problem {
    pub fn init(initial_state: node::Node) -> Problem {
        Problem {
            initial_state,
            operators: [Problem::operator_swap],
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
    fn operator_swap(node: &node::Node, i: usize, j: usize, algorithm: Algorithm) -> node::Node {
        let mut state = node.state.clone();
        state.swap(j, i);
        let new_node = node::Node::init(state, node.cutaways, node.g + 1, algorithm);
        new_node
    }
}
