use std::fmt::Arguments;
use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant};

// This struct stores important things about the currently running program
struct Runtime {
    log: Option<File>,
    trace: bool,
    time: bool,
    start_time: Option<Instant>,
    duration: Duration,
    nodes_expanded: u64,
    max_size: u64,
}

impl Runtime {
    fn print(&mut self, args: Arguments<'_>) {
        if let Some(s) = args.as_str() {
            if self.trace {
                println!("{}", s);
            }
            if self.log.is_some() {
                // unwrap
                self.log.as_mut().unwrap().write_all(s.as_bytes());
            }
        }
    }
    fn start_timer(&mut self) {
        self.start_time = Some(Instant::now());
    }
    fn end_timer(&mut self) {
        if self.start_time.is_some() {
            self.duration = self.start_time.unwrap().elapsed();
        }
    }
}
