use crate::CUTAWAYS;
use crate::LINE_SIZE;

use crate::runtime::Algorithm;
use std::cmp::Ordering;

#[derive(Clone, Hash)]
pub struct Node {
    pub state: [u32; LINE_SIZE + CUTAWAYS],
    pub cutaways: [usize; CUTAWAYS],
    pub zero_tiles: Vec<usize>,
    pub g: u32,
    pub h: u32,
}

impl Node {
    pub fn init(
        state: [u32; LINE_SIZE + CUTAWAYS],
        cutaways: [usize; CUTAWAYS],
        g: u32,
        algorithm: Algorithm,
    ) -> Node {
        let mut h: u32 = 0;
        // If we know where the zero tile is, then just use the given
        // Otherwise, we have to find it
        // Note that cutaways are a seperate type of zero tile
        let mut zero_tiles: Vec<usize> = Vec::new();
        for i in 0..LINE_SIZE + CUTAWAYS {
            if state[i] == 0 {
                zero_tiles.push(i);
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
            // Note that the heuristic only cares about
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
        println!("{}", cutaway_str);
        println!("{}", line_str);
        let result = format!("{}\n{}\n", cutaway_str, line_str);
        result
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_cost: u32 = self.g + self.h;
        let other_cost: u32 = other.g + other.h;
        if self_cost < other_cost {
            Ordering::Greater
        } else if other_cost > self_cost {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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
