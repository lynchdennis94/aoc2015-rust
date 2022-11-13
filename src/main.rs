mod problem_reader;
pub use problem_reader::*;

mod problem_one;
pub use problem_one::*;

fn main() {
    // Problem 1
    let problem_1_input = problem_reader::read_file("./src/problem_inputs/problem_1_input.txt");
    let problem_1a_output = problem_one::determine_floor(problem_1_input.clone());
    let problem_1b_output = problem_one::determine_negative_one_position(problem_1_input.clone());
    println!("Problem 1a: {problem_1a_output}");
    println!("Problem 1b: {problem_1b_output}");
}
