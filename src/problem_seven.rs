use std::collections::HashMap;

pub fn solve_7a(input: String) -> String {
    // Set up our operation map
    let operation_map = build_map(input);

    // Set up an amortized map so we can shortcut stuff
    let mut amortized_map = HashMap::<String, u16>::new();

    // Parse our map to find the answer
    return parse_command(&operation_map, &mut amortized_map, "a".to_string()).to_string();
}

pub fn solve_7b(input: String) -> String {
    // Set up our operation map
    let mut operation_map = build_map(input);

    // Set up an amortized map so we can shortcut stuff
    let mut amortized_map = HashMap::<String, u16>::new();

    // Find the value of 'a'
    let a_value = parse_command(&operation_map, &mut amortized_map, "a".to_string());

    // Override the operation for 'b' to be the value of a
    operation_map.insert("b".to_string(), a_value.to_string());

    // Reset the amortized map
    amortized_map = HashMap::<String, u16>::new();

    // Find the new value of 'a'
    return parse_command(&operation_map, &mut amortized_map, "a".to_string()).to_string();
}

fn build_map(commands: String) -> HashMap<String, String> {
    let mut operation_map = HashMap::<String, String>::new(); // Signal to opcode string

    // Get the commands
    let split_string = commands.split('\n').collect::<Vec<&str>>();

    // Build our map
    for command in split_string {
        let command_vec = command.split("->").collect::<Vec<&str>>();
        operation_map.insert(
            String::from(command_vec.get(1).unwrap().trim()), // Signal name
            String::from(command_vec.get(0).unwrap().trim()), // Signal operation
        );
    }

    return operation_map;
}

fn parse_command(
    operation_map: &HashMap<String, String>,
    amortized_map: &mut HashMap<String, u16>,
    signal: String,
) -> u16 {
    if amortized_map.contains_key(&signal) {
        // We've already solved this, just return
        return *amortized_map.get(&signal).unwrap();
    }

    // Otherwise, we need to do work on the opcodes
    if operation_map.contains_key(&signal) {
        // First, get the operation on the current signal
        let operation = operation_map.get(&signal).unwrap();

        // Parse out the operation
        let parsed_operation_vec = operation.split(' ').collect::<Vec<&str>>();
        if parsed_operation_vec.len() == 1 {
            // Simple return operation - send back the signal
            let input_signal = String::from(*parsed_operation_vec.get(0).unwrap());
            let answer = parse_command(operation_map, amortized_map, input_signal);
            amortized_map.insert(signal, answer);
            return answer;
        } else if parsed_operation_vec.len() == 2 {
            // NOT operation
            let opcode = String::from(*parsed_operation_vec.get(0).unwrap());
            let input_signal = String::from(*parsed_operation_vec.get(1).unwrap());

            if opcode.eq("NOT") {
                let answer = !parse_command(operation_map, amortized_map, input_signal);
                amortized_map.insert(signal, answer);
                return answer;
            } else {
                panic!("Cannot support operation {operation}");
            }
        } else if parsed_operation_vec.len() == 3 {
            // AND, OR, LSHIFT, RSHIFT
            let opcode = String::from(*parsed_operation_vec.get(1).unwrap());
            let signal_a = String::from(*parsed_operation_vec.get(0).unwrap());
            let signal_b = String::from(*parsed_operation_vec.get(2).unwrap());

            if opcode.eq("AND") {
                let answer = parse_command(operation_map, amortized_map, signal_a)
                    & parse_command(operation_map, amortized_map, signal_b);
                amortized_map.insert(signal, answer);
                return answer;
            } else if opcode.eq("OR") {
                let answer = parse_command(operation_map, amortized_map, signal_a)
                    | parse_command(operation_map, amortized_map, signal_b);
                amortized_map.insert(signal, answer);
                return answer;
            } else if opcode.eq("LSHIFT") {
                let answer = parse_command(operation_map, amortized_map, signal_a)
                    << parse_command(operation_map, amortized_map, signal_b);
                amortized_map.insert(signal, answer);
                return answer;
            } else if opcode.eq("RSHIFT") {
                let answer = parse_command(operation_map, amortized_map, signal_a)
                    >> parse_command(operation_map, amortized_map, signal_b);
                amortized_map.insert(signal, answer);
                return answer;
            } else {
                panic!("{opcode} isn't a valid opcode");
            }
        } else {
            panic!("Cannot support operation {operation}");
        }
    } else {
        // We've hit a number, return out
        return signal.parse::<u16>().unwrap();
    }
}
