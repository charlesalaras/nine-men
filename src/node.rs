use crate::CUTAWAYS;
use crate::LINE_SIZE;

pub struct Node {
    state: [u32; LINE_SIZE + CUTAWAYS],
    cutaways: [u32; CUTAWAYS],
    zero_tile: u32,
    g: u32,
    h: u32,
}

impl Board {
    fn init(state: [u32; LINE_SIZE + CUTAWAYS], cutaways: [u32; CUTAWAYS]) -> Node {}
}
