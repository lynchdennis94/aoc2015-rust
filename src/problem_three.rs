use std::collections::HashSet;

pub fn solve_3a(input: String) -> usize {
    return get_unique_trip_set(input).len();
}

pub fn solve_3b(input: String) -> usize {
    let mut santa_instructions: String = String::new();
    let mut robo_santa_instructions: String = String::new();

    // Break up the instructions into real and robo instructions
    for (char_index, char_value) in input.char_indices() {
        if char_index % 2 == 0 {
            // This is a 'real' santa instruction
            santa_instructions.push_str(&String::from(char_value));
        } else {
            robo_santa_instructions.push_str(&String::from(char_value));
        }
    }

    // Calculate their individual trip sets
    let real_santa_trip_set = get_unique_trip_set(santa_instructions);
    let robo_santa_trip_set = get_unique_trip_set(robo_santa_instructions);

    // Combine the sets (in case they both visit the same house) and return the solution
    return real_santa_trip_set.union(&robo_santa_trip_set).count();
}

/**
 * Gets the number of unique visits based on the input instructions
 * and returns a set with the unique coordinates visited
 */
fn get_unique_trip_set(input: String) -> HashSet<(i32, i32)> {
    let mut unique_trips: HashSet<(i32, i32)> = HashSet::new();
    let mut current_x = 0;
    let mut current_y = 0;

    // Set up the map to visit the current house
    unique_trips.insert((current_x, current_y));

    // Set up the map to track visits
    for character in input.chars() {
        match character {
            '>' => current_x += 1,
            '<' => current_x -= 1,
            '^' => current_y += 1,
            'v' => current_y -= 1,
            _ => panic!("Couldn't parse instructions - too much 'nog for the elves!"),
        }
        unique_trips.insert((current_x, current_y));
    }

    return unique_trips;
}
