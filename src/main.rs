use std::env;

mod problem_reader;
pub use problem_reader::*;

mod problem_one;
pub use problem_one::*;

mod problem_two;
pub use problem_two::*;

mod problem_three;
pub use problem_three::*;

mod problem_four;
pub use problem_four::*;

mod problem_five;
pub use problem_five::*;

mod problem_six;
pub use problem_six::*;

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
        1 => solve_problem(arg_index, problem_one::solve_1a, problem_one::solve_1b),
        2 => solve_problem(arg_index, problem_two::solve_2a, problem_two::solve_2b),
        3 => solve_problem(arg_index, problem_three::solve_3a, problem_three::solve_3b),
        4 => solve_problem(arg_index, problem_four::solve_4a, problem_four::solve_4b),
        5 => solve_problem(arg_index, problem_five::solve_5a, problem_five::solve_5b),
        6 => solve_problem(arg_index, problem_six::solve_6a, problem_six::solve_6b),
        other => {
            println!("Problem {other} not implemented");
        }
    }
}

fn solve_problem(
    problem_index: i32,
    part_a_solver: fn(String) -> String,
    part_b_solver: fn(String) -> String,
) {
    let filepath = format!("./src/problem_inputs/problem_{problem_index}_input.txt");
    let problem_input = problem_reader::read_file(filepath.as_str());
    //let problem_input = "turn off 499,499 through 500,500".to_string();
    let part_a_output = part_a_solver(problem_input.clone());
    let part_b_output = part_b_solver(problem_input.clone());
    println!("Part A Output: {part_a_output}");
    println!("Part B Output: {part_b_output}");
}
