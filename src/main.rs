use std::env;

mod problem_reader;
pub use problem_reader::*;

mod problem_one;
pub use problem_one::*;

mod problem_two;
pub use problem_two::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Need to define a problem to solve!");
    }
    let arg_index = args
        .get(1)
        .expect("No index defined")
        .parse::<i32>()
        .unwrap();

    match arg_index {
        1 => {
            // Problem 1
            let problem_1_input =
                problem_reader::read_file("./src/problem_inputs/problem_1_input.txt");
            let problem_1a_output = problem_one::solve_1a(problem_1_input.clone());
            let problem_1b_output = problem_one::solve_1b(problem_1_input.clone());
            println!("Problem 1a: {problem_1a_output}");
            println!("Problem 1b: {problem_1b_output}");
        }
        2 => {
            // Problem 2
            let problem_2_input =
                problem_reader::read_file("./src/problem_inputs/problem_2_input.txt");
            let problem_2a_output = problem_two::solve_2a(problem_2_input.clone());
            let problem_2b_output = problem_two::solve_2b(problem_2_input.clone());
            println!("Problem 2a: {problem_2a_output}");
            println!("Problem 2b: {problem_2b_output}");
        }
        _ => {
            println!("Problem not implemented");
        }
    }
}
