use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let result: u64 = read_to_string("../input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .map(get_power)
        .filter(|x| x.is_some())
        .map(Option::unwrap)
        .sum();

    println!("Result: {}", result);
}

fn get_power(s: String) -> Option<u64> {
    print!("{}", &s);

    let game_info_regex = Regex::new(r"^Game (?<game_number>\d+): (?<reveals>.+)$").unwrap();

    let Some(game_info) = game_info_regex.captures(&s) else {
        return None;
    };

    let game_number: &u64 = &game_info["game_number"].parse::<u64>().unwrap();
    let reveals: Vec<&str> = game_info["reveals"].split(";").map(str::trim).collect();
    let counts = get_max_counts(reveals.clone());
    let power = counts.0 * counts.1 * counts.2;

    println!("Game: {:?}", game_number);
    println!("Counts: {:?}", counts);
    println!("Power: {:?}\n", power);

    return Some(power);
}

fn get_max_counts(reveals: Vec<&str>) -> (u64, u64, u64) {
    let mut max_counts: (u64, u64, u64) = (0, 0, 0);

    for reveal in reveals {
        let cubes: Vec<&str> = reveal.split(",").map(str::trim).collect();
        let counts = count_cubes(cubes);
        if counts.0 > max_counts.0 {
            max_counts.0 = counts.0
        }
        if counts.1 > max_counts.1 {
            max_counts.1 = counts.1
        }
        if counts.2 > max_counts.2 {
            max_counts.2 = counts.2
        }
    }

    return max_counts;
}

// Returns a tuple of cube counts for (red, green, blue)
fn count_cubes(cubes: Vec<&str>) -> (u64, u64, u64) {
    let mut counts: (u64, u64, u64) = (0, 0, 0);

    for cube in cubes {
        let cube_data: Vec<&str> = cube.split(" ").map(str::trim).collect();

        let count = cube_data[0].parse::<u64>().unwrap();
        let color = cube_data[1];

        match color {
            "red" => counts.0 = count,
            "green" => counts.1 = count,
            "blue" => counts.2 = count,
            _ => println!("Uhhhh"),
        }
    }

    return counts;
}
