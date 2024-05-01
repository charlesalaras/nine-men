use crate::CUTAWAYS;
use crate::LINE_SIZE;

use crate::runtime::Algorithm;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct Node {
    pub state: [u32; LINE_SIZE + CUTAWAYS],
    pub cutaways: [usize; CUTAWAYS],
    pub zero_tile: Vec<u32>,
    pub g: u32,
    pub h: u32,
}

impl Node {
    pub fn init(
        state: [u32; LINE_SIZE + CUTAWAYS],
        cutaways: [usize; CUTAWAYS],
        g: u32,
        zero_tile: Option<usize>,
        algorithm: Algorithm,
    ) -> Node {
        let mut h: u32 = 0;
        let mut index: usize = 0;
        // If we know where the zero tile is, then just use the given
        // Otherwise, we have to find it
        // Note that cutaways are a seperate type of zero tile
        match zero_tile {
            Some(i) => index = i,
            None => {
                for i in 0..LINE_SIZE {
                    if state[i] == 0 {
                        index = i;
                        break;
                    }
                }
            }
        }
        // Calculate the given heuristic
        unsafe {
            match algorithm {
                crate::Algorithm::UniformCost => (),
                crate::Algorithm::MisplacedTile => {
                    for i in 0..LINE_SIZE + CUTAWAYS {
                        if state[i] != crate::GOAL_STATE[i] {
                            h = h + 1;
                        }
                    }
                }
                crate::Algorithm::ManhattanDist => {
                    for i in 0..LINE_SIZE {
                        let curr = crate::GOAL_STATE[i];
                    }
                    for i in 0..CUTAWAYS {
                        let curr = cutaways[i];
                    }
                }
            }
        }
        Node {
            state,
            cutaways,
            zero_tile: Vec::new(),
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
