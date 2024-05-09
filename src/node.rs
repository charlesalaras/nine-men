use crate::CUTAWAYS;
use crate::LINE_SIZE;

use crate::runtime::Algorithm;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/*
Node object to represent any given state of the puzzle:
- state: Array of size LINE_SIZE + CUTAWAYS. Note that the cutaways are placed at the back of the array.
- cutaways: Array of indices denoting where the cutaway exists in relation to the line.
- zero_tiles: Array of indices denoting which tiles are zero. Note that this should always be CUTAWAYS + 1 size.
- g: The cost function
- h: The heuristic function (This is zero for UCS)
*/
#[derive(Copy, Clone)]
pub struct Node {
    pub state: [u32; LINE_SIZE + CUTAWAYS],
    pub cutaways: [usize; CUTAWAYS],
    pub zero_tiles: [usize; CUTAWAYS + 1],
    pub g: u32,
    pub h: u32,
}

impl Node {
    // Constructs and returns a node
    pub fn init(
        state: [u32; LINE_SIZE + CUTAWAYS],
        cutaways: [usize; CUTAWAYS],
        g: u32,
        algorithm: Algorithm,
    ) -> Node {
        let mut h: u32 = 0;
        // Find all zero tiles
        let mut zero_tiles: [usize; CUTAWAYS + 1] = [0, 0, 0, 0];
        let mut j = 0;
        for i in 0..LINE_SIZE + CUTAWAYS {
            if state[i] == 0 {
                zero_tiles[j] = i;
                j += 1;
            }
        }
        // Calculate the given heuristic
        match algorithm {
            crate::Algorithm::UniformCost => (),
            crate::Algorithm::MisplacedTile => {
                for i in 0..LINE_SIZE + CUTAWAYS {
                    if state[i] != crate::GOAL_STATE[i] {
                        h = h + 1;
                    }
                }
            }
            // Note that the Manhattan heuristic only cares about
            // where the '1' tile is.
            crate::Algorithm::ManhattanDist => {
                for i in 0..LINE_SIZE {
                    if state[i] == 1 {
                        h = h + (i as u32);
                    }
                }
                // If '1' was not found in the line,
                // it may exist in the cutaways
                if h == 0 {
                    for i in 0..CUTAWAYS {
                        if state[LINE_SIZE + i] == 1 {
                            h = h + (cutaways[i] + 1) as u32;
                        }
                    }
                }
            }
        }

        Node {
            state,
            cutaways,
            zero_tiles,
            g,
            h,
        }
    }
    // Returns a string representing the node's state
    pub fn print(&self) -> String {
        let mut cutaway_str = String::new();
        let mut line_str = String::new();
        let mut j = 0;
        for i in 0..LINE_SIZE {
            if j < CUTAWAYS && i == self.cutaways[j] {
                cutaway_str.push(char::from_digit(self.state[LINE_SIZE + j], 10).unwrap());
                j = j + 1;
            } else {
                cutaway_str.push(' ');
            }
            line_str.push(char::from_digit(self.state[i], 10).unwrap());
            line_str.push(' ');
            cutaway_str.push(' ');
        }
        let result = format!("{}\n{}\n", cutaway_str, line_str);
        result
    }
}

/*
Comparison function required for the queue.
Note that by returning Ordering::Greater for lower f(n),
and Ordering::Less for higher f(n),
we are creating a min-queue, expanding nodes with
the lowest f(n) = g(n) + h(n)
*/
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_cost: u32 = self.g + self.h;
        let other_cost: u32 = other.g + other.h;
        if self_cost < other_cost {
            Ordering::Greater
        } else if other_cost > self_cost {
            Ordering::Less
        } else {
            // Tie breaker: Use the lower cost
            /*if self.g < other.g {
                Ordering::Greater
            } else {
                Ordering::Less
            }*/
            Ordering::Equal
        }
    }
}

/*
Hash implementation. This simply takes the state and hashes it.
This is important because hashing the entire node leads to duplicates.
Example: Same state, but different costs.
*/
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.hash(state);
    }
}

/*
PartialOrd is required for the queue, it simply compares using the above Ord
*/
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/*
PartialEq to determine Node equality.
The function simply checks if the state is equivalent.
Eq (defined below), is based off of this PartialEq.
This is required for the queue.
*/
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..LINE_SIZE + CUTAWAYS {
            if self.state[i] != other.state[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for Node {}
