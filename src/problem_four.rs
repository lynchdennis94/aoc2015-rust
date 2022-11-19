use md5;

pub fn solve_4a(input: String) -> String {
    iterate_over_options(input, "00000")
}

pub fn solve_4b(input: String) -> String {
    iterate_over_options(input, "000000")
}

fn iterate_over_options(input: String, leading_string: &str) -> String {
    // "Brute Force" way - iterate over all possible values in the chunk
    for possible_answer in 1..=std::i32::MAX {
        let current_string = format!("{input}{possible_answer}");
        let hashed_answer = derive_hash(current_string);
        if hashed_answer.starts_with(leading_string) {
            return format!("{possible_answer}");
        }
    }

    // We didn't find anything, return -1
    return "-1".into();
}

fn derive_hash(input: String) -> String {
    let digest = md5::compute(input);
    format!("{:x}", digest)
}
