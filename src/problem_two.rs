pub fn solve_2a(input: String) -> i32 {
    let mut total_surface_area = 0;
    let packages = input.split("\n");
    for package in packages {
        total_surface_area += get_wrapping_for_single_package(package);
    }
    return total_surface_area;
}

pub fn solve_2b(input: String) -> i32 {
    let mut total_ribbon_length = 0;
    let packages = input.split("\n");
    for package in packages {
        total_ribbon_length += get_ribbons_for_single_package(package);
    }
    return total_ribbon_length;
}

fn get_package_dimensions(package: &str) -> Vec<i32> {
    let dimensions = package.split("x");
    let mut dimension_vec: Vec<i32> = Vec::new();
    for dimension in dimensions {
        let dimension_value = dimension.parse::<i32>().unwrap();
        dimension_vec.push(dimension_value);
    }

    return dimension_vec;
}

fn get_wrapping_for_single_package(package: &str) -> i32 {
    // Break out the individual package definition into it's parts
    let dimension_vec = get_package_dimensions(package);

    // Figure out the package surface area AND smallest side
    let mut smallest_area = -1;
    let mut total_area = 0;
    let mut multiplication_tuples_vec = Vec::new();
    multiplication_tuples_vec.push((0, 1));
    multiplication_tuples_vec.push((0, 2));
    multiplication_tuples_vec.push((1, 2));
    for multiplication_tuple in multiplication_tuples_vec {
        let first_dimension = dimension_vec
            .get(multiplication_tuple.0)
            .expect("Cannot retrieve first dimension");
        let second_dimension = dimension_vec
            .get(multiplication_tuple.1)
            .expect("Cannot retrieve second dimension");
        let current_area = first_dimension * second_dimension;
        if smallest_area == -1 || current_area < smallest_area {
            smallest_area = current_area;
        }
        total_area += 2 * current_area;
    }

    return total_area + smallest_area;
}

fn get_ribbons_for_single_package(package: &str) -> i32 {
    let dimension_vec = get_package_dimensions(package);
    let mut dimension_tuple_vec = Vec::new();
    dimension_tuple_vec.push((0, 1));
    dimension_tuple_vec.push((0, 2));
    dimension_tuple_vec.push((1, 2));

    let mut smallest_perimeter = -1;
    for dimension_tuple in dimension_tuple_vec {
        let first_dimension = dimension_vec
            .get(dimension_tuple.0)
            .expect("Couldn't retrieve first dimension");
        let second_dimension = dimension_vec
            .get(dimension_tuple.1)
            .expect("Couldn't retrieve second dimension");
        let current_perimeter = 2 * first_dimension + 2 * second_dimension;
        if smallest_perimeter == -1 || current_perimeter < smallest_perimeter {
            smallest_perimeter = current_perimeter;
        }
    }
    return smallest_perimeter
        + dimension_vec.get(0).expect("Cannot get first dimension")
            * dimension_vec.get(1).expect("Cannot get second dimension")
            * dimension_vec.get(2).expect("Cannot get third dimension");
}
