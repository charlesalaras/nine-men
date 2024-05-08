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
/*
Runtime object to store important statistics and members to track the program.
Note that this is required due to Rust's aversion to global variables.
- log_name: A string representing the file name (or None if we do not need to write to a file)
- log: File object if writing to a file (or None if we do not need to write to a file)
- trace: Determines output to stdout
- time: Determines timing of search
- start_time: The Instant that the search began (or None if not timing / has not started)
- duration: Total running time of search (or None if not timing / incomplete)
- nodes_expanded: Total number of nodes expanded during search
- max_size: Maximum size of queue during search
- search: Heuristic being used in the search
- seen: A HashSet of nodes, used by the expand function (see search.rs) to store already expanded nodes
Note that seen exists in the Runtime struct because
Runtime is always available to the search and expand
functions. Problem is only passed in to search, not expand.
*/
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
    // Construct a file (if needed) and return a Runtime
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
    /*
    Wrapper function to write formatted string
    to stdout (if needed) and file (if needed)

    Note that file output may be mangled due to
    coloring of formatted strings.
    */
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
