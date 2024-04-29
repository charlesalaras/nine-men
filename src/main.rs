mod node;
mod problem;
mod runtime;
mod search;

use clap::Parser;
use std::io;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Output trace of search to stdout
    #[arg(long, default_value_t = true)]
    trace: bool,
    /// Path of file to write trace to
    #[arg(short, long)]
    logfile: String,
    /// Measure and output search time
    #[arg(short, long, default_value_t = false)]
    time: bool,
}

enum Algorithm {
    UniformCost,
    MisplacedTile,
    ManhattanDist,
}

/*
Parameters for problem:
- LINE_SIZE: Specifies the length of the line
- CUTAWAYS: Specifies how many recesses exist in the line that can be moved into
- SEARCH_ALGO: Specifies the heuristic to use (see above enum Algorithm)
- GOAL_STATE: Specifies what the final solution should look like
*/
const LINE_SIZE: usize = 10;
const CUTAWAYS: usize = 3;
const OPERATORS: usize = 4;
static mut SEARCH_ALGO: Algorithm = Algorithm::UniformCost;
static GOAL_STATE: [u32; LINE_SIZE + CUTAWAYS] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0];

fn main() {
    let args = Args::parse();
    println!("CS205 Spring 2024: Charles Alaras' Nine Men in a Trench Solver\n");
    loop {
        println!("Select a number (1, 2, 3) to define the algorithm heuristics:");
        println!("(1) Uniform Cost Search");
        println!("(2) Misplaced Tile Heuristic");
        println!("(3) Manhattan Distance Heuristic");
        let mut str = String::new();
        io::stdin()
            .read_line(&mut str)
            .expect("Failed to parse input");
        let selection = str.trim().parse::<u32>();
        if selection.is_ok() {
            match selection.unwrap() {
                1 => {
                    unsafe {
                        println!("You selected: Uniform Cost");
                        SEARCH_ALGO = Algorithm::UniformCost;
                    }
                    break;
                }
                2 => {
                    unsafe {
                        println!("You selected: Misplaced Tile");
                        SEARCH_ALGO = Algorithm::MisplacedTile;
                    }
                    break;
                }
                3 => {
                    unsafe {
                        println!("You selected: Manhattan Distance");
                        SEARCH_ALGO = Algorithm::ManhattanDist;
                    }
                    break;
                }
                _ => {
                    println!("Please select a number between (1-3)\n");
                }
            }
        } else if selection.is_err() {
            println!("ERROR: {}", selection.err().unwrap());
        }
    }
    // Check if we should be timing
    let node: node::Node = node::Node::init(
        [0, 2, 3, 4, 5, 6, 7, 8, 9, 1, 0, 0, 0],
        [3, 5, 7],
        0,
        Some(0),
    );
    let problem: problem::Problem = problem::Problem::init(node);
    let result = search::search(problem, search::queueing_function);
    match result {
        Some(x) => {
            println!("Goal state found!");
            x.print();

            println!("\nSolution depth was {}", 5);
            println!("Number of nodes expanded: {}", 5);
            println!("Max queue size: {}", 5);
        }
        None => {
            println!("Failure");
        }
    }
}
