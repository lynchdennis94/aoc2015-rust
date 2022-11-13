pub fn solve_1a(input: String) -> i32 {
    let mut floor_counter: i32 = 0;
    for floor_char in input.chars() {
        floor_counter += translate_char_to_position(floor_char);
    }
    return floor_counter;
}

pub fn solve_1b(input: String) -> i32 {
    let mut floor_counter: i32 = 0;
    let mut current_position_counter = 0;
    for floor_char in input.chars() {
        current_position_counter += 1;
        floor_counter += translate_char_to_position(floor_char);
        if floor_counter == -1 {
            break;
        }
    }

    return current_position_counter;
}

fn translate_char_to_position(input_char: char) -> i32 {
    if input_char == '(' {
        return 1;
    } else if input_char == ')' {
        return -1;
    } else {
        return 0;
    }
}
