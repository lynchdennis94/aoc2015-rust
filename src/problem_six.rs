pub fn solve_6a(input: String) -> String {
    // Set up the grid, assume all lights are off
    let mut lighting_grid = vec![vec![false; 1000]; 1000];

    // Follow all of the instructions
    for command in input.split('\n').collect::<Vec<&str>>() {
        let parsed_command = parse_command(String::from(command));
        let command_string = parsed_command.0;

        if command_string.eq("toggle") {
            change_grid_values(
                &mut lighting_grid,
                parsed_command.1,
                parsed_command.2,
                |input: bool| -> bool { !input },
            )
        } else if command_string.eq("turn on") {
            change_grid_values(
                &mut lighting_grid,
                parsed_command.1,
                parsed_command.2,
                |_input: bool| -> bool { true },
            )
        } else if command_string.eq("turn off") {
            change_grid_values(
                &mut lighting_grid,
                parsed_command.1,
                parsed_command.2,
                |_input: bool| -> bool { false },
            )
        }
    }

    // Count all the lights that are on
    let mut light_counter = 0;
    for light_row in lighting_grid {
        light_counter += light_row.into_iter().filter(|v| *v).count();
    }
    return light_counter.to_string();
}

pub fn solve_6b(input: String) -> String {
    // Set up the grid, assume all lights are at brightness 0
    let mut lighting_grid = vec![vec![0; 1000]; 1000];

    // Follow the instructions
    for command in input.split('\n').collect::<Vec<&str>>() {
        let parsed_command = parse_command(String::from(command));
        let command_string = parsed_command.0;

        if command_string.eq("toggle") {
            change_grid_brightness(
                &mut lighting_grid,
                parsed_command.1,
                parsed_command.2,
                |input: i32| -> i32 { input + 2 },
            )
        } else if command_string.eq("turn on") {
            change_grid_brightness(
                &mut lighting_grid,
                parsed_command.1,
                parsed_command.2,
                |input: i32| -> i32 { input + 1 },
            )
        } else if command_string.eq("turn off") {
            change_grid_brightness(
                &mut lighting_grid,
                parsed_command.1,
                parsed_command.2,
                |input: i32| -> i32 {
                    if input == 0 {
                        input
                    } else {
                        input - 1
                    }
                },
            )
        }
    }

    // Count the total brightness of each light
    let mut total_brightness = 0;
    for light_row in lighting_grid {
        total_brightness += light_row.into_iter().sum::<i32>();
    }
    return total_brightness.to_string();
}

fn parse_command(command: String) -> (String, (i32, i32), (i32, i32)) {
    let split_string = command.split(" ").collect::<Vec<&str>>();
    let mut parsed_command = (*split_string.get(0).unwrap()).clone().to_string();
    let mut coord_index = 1;

    // Differentiate between the 'toggle' and 'turn on/off' commands
    if parsed_command.eq(&String::from("turn")) {
        parsed_command.push_str(" ");
        parsed_command.push_str(split_string.get(1).unwrap());
        coord_index = 2;
    }

    // Split out the start and end coordinates
    let start_coordinates =
        parse_coord_string_into_tuple(split_string.get(coord_index).unwrap().clone().to_string());
    let end_coordinates = parse_coord_string_into_tuple(
        split_string
            .get(coord_index + 2)
            .unwrap()
            .clone()
            .to_string(),
    );
    return (
        parsed_command.to_ascii_lowercase(),
        start_coordinates,
        end_coordinates,
    );
}

fn parse_coord_string_into_tuple(coord_string: String) -> (i32, i32) {
    let split_string = coord_string.split(',').collect::<Vec<&str>>();
    return (
        split_string.get(0).unwrap().parse().unwrap(),
        split_string.get(1).unwrap().parse().unwrap(),
    );
}

fn change_grid_values(
    lighting_grid: &mut Vec<Vec<bool>>,
    start: (i32, i32),
    stop: (i32, i32),
    operation: fn(bool) -> bool,
) {
    let (start_x, start_y) = start;
    let (stop_x, stop_y) = stop;
    for current_x in start_x..=stop_x {
        for current_y in start_y..=stop_y {
            lighting_grid[current_x as usize][current_y as usize] =
                operation(lighting_grid[current_x as usize][current_y as usize]);
        }
    }
}

fn change_grid_brightness(
    lighting_grid: &mut Vec<Vec<i32>>,
    start: (i32, i32),
    stop: (i32, i32),
    operation: fn(i32) -> i32,
) {
    let (start_x, start_y) = start;
    let (stop_x, stop_y) = stop;
    for current_x in start_x..=stop_x {
        for current_y in start_y..=stop_y {
            lighting_grid[current_x as usize][current_y as usize] =
                operation(lighting_grid[current_x as usize][current_y as usize]);
        }
    }
}
