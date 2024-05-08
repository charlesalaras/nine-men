use crate::CUTAWAYS;
use crate::LINE_SIZE;
use crate::OPERATORS;

use crate::node;
use crate::runtime::Algorithm;

/*
Problem object to represent the entire problem:
- initial_state: Node to represent start of problem (see node.rs for details)
- operators: Array of size OPERATORS representing function pointers to different mutating operators.
*/
pub struct Problem {
    pub initial_state: node::Node,
    pub operators: [fn(&node::Node, usize, usize, Algorithm) -> node::Node; OPERATORS],
}

impl Problem {
    // Constructs using state and given operators (1 in this case)
    pub fn init(initial_state: node::Node) -> Problem {
        Problem {
            initial_state,
            operators: [Problem::operator_swap],
        }
    }
    // Tests whether a passed in state is the goal
    pub fn goal_test(state: [u32; CUTAWAYS + LINE_SIZE]) -> bool {
        let mut curr: u32 = 1;
        for i in 1..LINE_SIZE {
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
    /*
    The operator function for the problem.
    There's technically four operators:
    - Slide zero tile left
    - Slide zero tile right
    - Slide zero tile up
    - Slide zero tile down
    But programatically, we can roll all operators into
    this one function given the right indices.
    The expand function (search.rs),
    verifies what swaps need to / can occur.
    */
    fn operator_swap(node: &node::Node, i: usize, j: usize, algorithm: Algorithm) -> node::Node {
        let mut state = node.state.clone();
        state.swap(j, i);
        let new_node = node::Node::init(state, node.cutaways, node.g + 1, algorithm);
        new_node
    }
}
