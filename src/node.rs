use crate::CUTAWAYS;
use crate::LINE_SIZE;
use crate::SEARCH_ALGO;

use std::cmp::Ordering;

pub struct Node {
    pub state: [u32; LINE_SIZE + CUTAWAYS],
    pub cutaways: [u32; CUTAWAYS],
    pub zero_tile: usize,
    pub g: u32,
    pub h: u32,
}

impl Node {
    pub fn init(
        state: [u32; LINE_SIZE + CUTAWAYS],
        cutaways: [u32; CUTAWAYS],
        zero_tile: Option<usize>,
    ) -> Node {
        let mut g: u32 = 0;
        let mut h: u32 = 0;
        let mut index: usize = 0;
        match zero_tile {
            Some(i) => index = i,
            None => {
                for i in 0..LINE_SIZE {
                    if state[i] == 0 {
                        index = i;
                    }
                }
            }
        }
        match SEARCH_ALGO {
            UniformCost => (),
            MisplacedTile => {}
            ManhattanDist => {}
        }
        Node {
            state,
            cutaways,
            zero_tile: index,
            g,
            h,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match SEARCH_ALGO {
            UniformCost => println!("Test"),
            MisplacedTile => println!("Test2"),
            ManhattanDist => println!("Test3"),
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
