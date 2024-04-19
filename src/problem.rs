use crate::CUTAWAYS;
use crate::LINE_SIZE;

use crate::node;

pub struct Problem {
    pub initial_state: node::Node,
    pub operators: [fn(usize) -> node::Node; 2],
}

impl Problem {
    pub fn goal_test(state: [u32; CUTAWAYS + LINE_SIZE]) -> bool {
        let mut curr: u32 = 1;
        for i in 1..LINE_SIZE - 1 {
            if state[i - 1] != curr {
                return false;
            }
            curr += 1;
        }
        true
    }
    fn operator_swap(zero_tile: usize) -> node::Node {}
    fn operator_cutaway(index: usize) -> node::Node {}
}
