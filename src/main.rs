mod node;
mod problem;
mod runtime;
mod search;

use crate::runtime::Algorithm;
use clap::Parser;
use colored::Colorize;
use std::io;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Output trace of search to stdout
    #[arg(long, default_value_t = false)]
    no_trace: bool,
    /// Path of file to write trace to
    #[arg(short, long)]
    logfile: Option<String>,
    /// Measure and output search time
    #[arg(short, long, default_value_t = false)]
    time: bool,
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
const OPERATORS: usize = 1;
const GOAL_STATE: [u32; LINE_SIZE + CUTAWAYS] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0];

fn main() {
    let args = Args::parse();
    let algorithm: runtime::Algorithm;
    if (!args.no_trace || args.logfile.is_some()) && args.time {
        println!("{}", "WARNING: Tracing program negatively impacts running time of search. Time statistics may not be accurate.".red().bold());
    }
    println!(
        "{}",
        "CS205 Spring 2024: Charles Alaras' Nine Men in a Trench Solver\n"
            .white()
            .bold()
    );
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
                    println!("You selected: {}", "Uniform Cost".purple());
                    algorithm = Algorithm::UniformCost;
                    break;
                }
                2 => {
                    println!("You selected: {}", "Misplaced Tile".purple());
                    algorithm = Algorithm::MisplacedTile;
                    break;
                }
                3 => {
                    println!("You selected: {}", "Manhattan Distance".purple());
                    algorithm = Algorithm::ManhattanDist;
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
    let filename: Option<String> = args.logfile;
    let mut runtime: runtime::Runtime =
        runtime::Runtime::init(!args.no_trace, args.time, algorithm, &filename);
    let node: node::Node = node::Node::init(
        [0, 2, 3, 4, 5, 6, 7, 8, 9, 1, 0, 0, 0],
        [3, 5, 7],
        0,
        runtime.search,
    );

    let problem: problem::Problem = problem::Problem::init(node);
    let result;

    if runtime.time {
        runtime.start_timer();
        result = search::search(problem, search::queueing_function, &mut runtime);
        runtime.end_timer();
    } else {
        result = search::search(problem, search::queueing_function, &mut runtime);
    }
    match result {
        Some(x) => {
            println!("{}", "Goal state found!".green().bold());
            println!("{}", x.print());

            println!("Solution depth was {}", x.g);
            println!("Number of nodes expanded: {}", runtime.nodes_expanded);
            println!("Max queue size: {}", runtime.max_size);
        }
        None => {
            println!("{}", "Failure".red().bold());
        }
    }
}
