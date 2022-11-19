use std::collections::HashMap;

pub fn solve_5a(input: String) -> String {
    let mut nice_string_counter = 0;
    for sample_string in input.split('\n') {
        if is_string_nice(String::from(sample_string)) {
            nice_string_counter += 1;
        }
    }
    return nice_string_counter.to_string();
}

pub fn solve_5b(input: String) -> String {
    let mut nice_string_counter = 0;
    for sample_string in input.split('\n') {
        if is_string_nice_new(String::from(sample_string)) {
            nice_string_counter += 1;
        }
    }
    return nice_string_counter.to_string();
}

fn is_string_nice(input: String) -> bool {
    let mut vowel_counter = 0;
    let mut found_double_letter = false;
    let mut found_forbidden_sequence = false;
    let chars: Vec<char> = input.chars().collect();
    for char_index in 0..chars.len() {
        let char_value = chars.get(char_index).unwrap();
        if is_vowel(char_value) {
            vowel_counter += 1;
        }

        if char_index > 0 {
            // Get the previous letter
            let previous_char_value = chars.get(char_index - 1).unwrap();
            // Double letter
            if char_value == previous_char_value {
                found_double_letter = true;
            }

            // Check if we've just found a 'forbidden sequence'
            if is_forbidden_sequence(format!("{previous_char_value}{char_value}")) {
                found_forbidden_sequence = true;
            }
        }
    }
    return vowel_counter >= 3 && found_double_letter && !found_forbidden_sequence;
}

fn is_string_nice_new(input: String) -> bool {
    let mut pairs_map: HashMap<String, usize> = HashMap::new(); // Maps pairs to the end index of the pair
    let mut found_trio = false;
    let mut found_pairs = false;
    let chars: Vec<char> = input.chars().collect();

    for char_index in 0..chars.len() {
        // Get the current 'pair'
        if char_index >= 1 {
            let current_pair = String::from(&input[char_index - 1..=char_index]);
            if pairs_map.contains_key(&current_pair) {
                if char_index - 2 >= *pairs_map.get(&current_pair).unwrap() {
                    found_pairs = true;
                }
            } else {
                // We only insert the first occurrence - future occurrences won't need to be stored,
                // since we only care about the appearance of a distinct pair (not how close/far it is)
                pairs_map.insert(current_pair, char_index);
            }
        }
        // See if we have a trio
        if char_index >= 2 && is_trio(String::from(&input[char_index - 2..=char_index])) {
            found_trio = true;
        }
    }
    return found_trio && found_pairs;
}

fn is_vowel(char_value: &char) -> bool {
    ['a', 'e', 'i', 'o', 'u'].contains(char_value)
}

fn is_forbidden_sequence(current_sequence: String) -> bool {
    ["ab", "cd", "pq", "xy"].contains(&current_sequence.as_str())
}

fn is_trio(input: String) -> bool {
    if input.len() != 3 {
        return false; // Can't have a trio with more or less than 3 chars
    } else {
        return input.chars().nth(0).unwrap() == input.chars().nth(2).unwrap()
            && input.chars().nth(0).unwrap() != input.chars().nth(1).unwrap();
    }
}
