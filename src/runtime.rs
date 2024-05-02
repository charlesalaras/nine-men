use crate::node;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant};

#[derive(Copy, Clone)]
pub enum Algorithm {
    UniformCost,
    MisplacedTile,
    ManhattanDist,
}
// This struct stores important things about the currently running program
pub struct Runtime {
    pub log_name: Option<String>,
    log: Option<File>,
    pub trace: bool,
    pub time: bool,
    start_time: Option<Instant>,
    pub duration: Option<Duration>,
    pub nodes_expanded: u64,
    pub max_size: usize,
    pub search: Algorithm,
    pub seen: HashSet<node::Node>,
}

impl Runtime {
    pub fn init(trace: bool, time: bool, search: Algorithm, log_name: &Option<String>) -> Runtime {
        let mut file = None;
        if log_name.is_some() {
            file =
                Some(File::create(log_name.as_ref().unwrap()).expect(
                    format!("Could not open file: {}", log_name.as_ref().unwrap()).as_str(),
                ));
        }
        Runtime {
            log_name: log_name.clone(),
            log: file,
            trace,
            time,
            start_time: None,
            duration: None,
            nodes_expanded: 0,
            max_size: 0,
            search,
            seen: HashSet::new(),
        }
    }
    pub fn print(&mut self, s: String) {
        if self.trace {
            print!("{}", s);
        }
        if self.log_name.is_some() {
            self.log.as_mut().unwrap().write_all(s.as_bytes()).expect(
                format!("Could not write file: {}", self.log_name.as_ref().unwrap()).as_str(),
            );
        }
    }
    pub fn start_timer(&mut self) {
        if self.time {
            self.start_time = Some(Instant::now());
        }
    }
    pub fn end_timer(&mut self) {
        self.duration = Some(self.start_time.unwrap().elapsed());
    }
}
