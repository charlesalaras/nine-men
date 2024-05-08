mod node;
mod problem;
mod runtime;
mod search;

use crate::runtime::Algorithm;
use clap::Parser;
use colored::Colorize;
use std::io;

/*
Arguments for command line. Run the program with `--help` for details.
*/
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Disable trace to stdout
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
Prompts creation of  and returns a state array for initialization.
Note that this function does not verify a state is solvable.
*/
fn create_state() -> [u32; LINE_SIZE + CUTAWAYS] {
    let mut state: [u32; LINE_SIZE + CUTAWAYS] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 1, 0, 0, 0];
    println!(
        "{}",
        "WARNING: Ensure that the entered state is solvable.".yellow()
    );
    let mut i = 0;
    let mut zeroes = 0;
    while i < LINE_SIZE + CUTAWAYS {
        if i < LINE_SIZE {
            print!("Enter value for the {}-th tile: ", i);
        } else {
            print!("Enter value for the {}-th cutaway: ", i - LINE_SIZE);
        }
        let mut str = String::new();
        io::stdin()
            .read_line(&mut str)
            .expect("Failed to parse input");
        let tile = str.trim().parse::<u32>();
        if tile.is_ok() {
            let value = tile.unwrap();
            if value == 0 && zeroes >= CUTAWAYS {
                println!("Too many empty tiles have been placed! Please choose a valid tile");
            }
            if value < 10 {
                state[i] = value;
                i = i + 1;
                if value == 0 {
                    zeroes = zeroes + 1;
                }
            } else {
                println!("Please enter a valid integer between 0-9");
            }
        } else if tile.is_err() {
            println!("ERROR: {}", tile.err().unwrap());
        }
    }
    state
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
        println!("{}", "WARNING: Tracing program negatively may impact running time of search. Time statistics may not be accurate.".red().bold());
    }
    println!(
        "{}",
        "CS205 Spring 2024: Charles Alaras' Nine Men in a Trench Solver\n"
            .white()
            .bold()
    );
    let mut state: [u32; LINE_SIZE + CUTAWAYS] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 1, 0, 0, 0];
    // Prompt for state creation
    loop {
        println!("Select a number (1, 2) to define the starting state:");
        println!("(1) Use the default initial state");
        println!("(2) Enter an intermediate state");
        let mut str = String::new();
        io::stdin()
            .read_line(&mut str)
            .expect("Failed to parse input");
        let selection = str.trim().parse::<u32>();
        if selection.is_ok() {
            match selection.unwrap() {
                1 => {
                    break;
                }
                2 => {
                    state = create_state();
                    break;
                }
                _ => {
                    println!("Please enter '1' or '2'\n");
                }
            }
        } else if selection.is_err() {
            println!("ERROR: {}", selection.err().unwrap());
        }
    }
    // Prompt for algorithm heuristic
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
    // Set up runtime (see `runtime.rs`) and the initial state.
    let mut runtime: runtime::Runtime =
        runtime::Runtime::init(!args.no_trace, args.time, algorithm, &filename);
    let node: node::Node = node::Node::init(state, [3, 5, 7], 0, runtime.search);

    let problem: problem::Problem = problem::Problem::init(node);
    let result;

    if runtime.time {
        // Measures duration of search
        runtime.start_timer();
        result = search::search(problem, search::queueing_function, &mut runtime);
        runtime.end_timer();
        println!("Time: {:.2}", runtime.duration.unwrap().as_secs_f64());
    } else {
        // Search without timing
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
